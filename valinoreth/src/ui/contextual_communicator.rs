//! [`ContextualCommunicator`] — wraps any communicator and prepends accumulated
//! game knowledge to every elicitation prompt.
//!
//! # Problem
//!
//! Each elicitation call is stateless — the communicator does not carry context
//! from one sampling request to the next.  When a combatant's turn begins, the
//! game state must be injected into the prompt so the player or agent can make
//! an informed decision.
//!
//! # Solution
//!
//! [`ContextualCommunicator`] wraps any inner communicator and maintains a
//! growing [`KnowledgeCache`].  Each prompt is prefixed with all accumulated
//! knowledge so the player sees the current combat state in every interaction.
//!
//! The game loop pushes a [`CombatStateView`] preamble into the cache before
//! eliciting each decision.  The cache is cleared between turns so stale state
//! does not accumulate.
//!
//! [`CombatStateView`]: crate::CombatStateView

use elicitation::{
    ElicitCommunicator, ElicitResult, ElicitationContext, StyleContext, StyleMarker,
};
use std::sync::{Arc, Mutex};
use tracing::instrument;

/// Accumulated game knowledge injected into every subsequent prompt.
///
/// Append-only within a turn; call [`KnowledgeCache::clear`] between turns.
#[derive(Debug, Clone, Default)]
pub struct KnowledgeCache {
    entries: Vec<String>,
}

impl KnowledgeCache {
    /// Append a new entry (e.g. the current [`CombatStateView`] preamble).
    ///
    /// [`CombatStateView`]: crate::CombatStateView
    pub fn push(&mut self, entry: impl Into<String>) {
        self.entries.push(entry.into());
    }

    /// Remove all entries (call between turns to avoid stale state).
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    fn format_preamble(&self) -> String {
        if self.entries.is_empty() {
            return String::new();
        }
        let mut preamble = String::from("[Game context]\n");
        for (i, entry) in self.entries.iter().enumerate() {
            preamble.push_str(&format!("{}. {}\n", i + 1, entry));
        }
        preamble.push('\n');
        preamble
    }
}

/// Thread-safe handle to a shared knowledge cache.
pub type SharedKnowledge = Arc<Mutex<KnowledgeCache>>;

/// Creates a new, empty shared knowledge cache.
#[instrument]
pub fn knowledge_cache() -> SharedKnowledge {
    Arc::new(Mutex::new(KnowledgeCache::default()))
}

/// Wraps any [`ElicitCommunicator`], prepending accumulated knowledge to
/// every prompt before delegating.
///
/// The owning game loop pushes context entries via [`SharedKnowledge`]
/// before calling into player decision methods.  The communicator reads
/// those entries and prepends them, so the player always sees up-to-date
/// game state without the game loop having to format every prompt manually.
#[derive(Clone)]
pub struct ContextualCommunicator<C> {
    inner: C,
    knowledge: SharedKnowledge,
}

impl<C> ContextualCommunicator<C> {
    /// Wrap `inner` with a shared knowledge cache.
    pub fn new(inner: C, knowledge: SharedKnowledge) -> Self {
        Self { inner, knowledge }
    }
}

impl<C: ElicitCommunicator + Clone> ElicitCommunicator for ContextualCommunicator<C> {
    /// Prepend accumulated knowledge to `prompt`, then delegate to inner.
    #[instrument(skip(self), level = "debug", fields(prompt_len = prompt.len()))]
    fn send_prompt(
        &self,
        prompt: &str,
    ) -> impl std::future::Future<Output = ElicitResult<String>> + Send {
        let preamble = {
            let cache = self.knowledge.lock().unwrap();
            cache.format_preamble()
        };
        let enriched = if preamble.is_empty() {
            prompt.to_string()
        } else {
            format!("{preamble}{prompt}")
        };
        let inner = self.inner.clone();
        tracing::debug!(prompt_len = enriched.len(), "Sending enriched prompt");
        async move { inner.send_prompt(&enriched).await }
    }

    /// Pass tool calls through to the inner communicator unchanged.
    #[instrument(skip(self, params), level = "debug", fields(tool = %params.name))]
    fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParams,
    ) -> impl std::future::Future<
        Output = Result<rmcp::model::CallToolResult, rmcp::service::ServiceError>,
    > + Send {
        self.inner.call_tool(params)
    }

    fn style_context(&self) -> &StyleContext {
        self.inner.style_context()
    }

    fn elicitation_context(&self) -> &ElicitationContext {
        self.inner.elicitation_context()
    }

    fn with_style<
        T: 'static,
        S: StyleMarker + elicitation::style::ElicitationStyle + 'static,
    >(
        &self,
        style: S,
    ) -> Self {
        Self {
            inner: self.inner.with_style::<T, S>(style),
            knowledge: self.knowledge.clone(),
        }
    }
}
