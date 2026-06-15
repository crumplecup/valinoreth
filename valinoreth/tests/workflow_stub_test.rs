//! Smoke tests for the combat workflow using a stub communicator.
//!
//! [`StubCommunicator`] always returns `"1"` (first option) without calling
//! any LLM or terminal API.  This exercises the complete
//! `CombatWorkflow::run()` path — VSM transitions, elicitation calls,
//! channel flow — without burning money or requiring a terminal.
//!
//! A separate section (gated behind `frontend-ratatui`) tests the
//! `ChatCommunicator` channel handoff independently.

use elicitation::{
    ElicitCommunicator, ElicitResult, ElicitationContext, StyleContext, StyleMarker,
};
use rmcp::model::{CallToolResult, Content};
use tokio::sync::mpsc;
use valinoreth::{
    AttributeType, CharacterDescriptorBuilder, CombatWorkflow, DerivedStatsDescriptor, GameMaster,
    Player, SkillDescriptorBuilder, SkillDifficulty,
};

// ── Tracing ───────────────────────────────────────────────────────────────────

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("valinoreth=debug")),
        )
        .with_test_writer()
        .try_init();
}

// ── StubCommunicator ──────────────────────────────────────────────────────────

/// A no-op [`ElicitCommunicator`] that picks option `"1"` for every prompt.
///
/// Use this to drive `CombatWorkflow` without a terminal or LLM dependency.
#[derive(Clone)]
struct StubCommunicator {
    style_ctx: StyleContext,
    elicit_ctx: ElicitationContext,
}

impl StubCommunicator {
    fn new() -> Self {
        Self {
            style_ctx: StyleContext::default(),
            elicit_ctx: ElicitationContext::default(),
        }
    }
}

impl ElicitCommunicator for StubCommunicator {
    async fn send_prompt(&self, _prompt: &str) -> ElicitResult<String> {
        Ok("1".to_string())
    }

    async fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParams,
    ) -> Result<rmcp::model::CallToolResult, rmcp::service::ServiceError> {
        // Handle elicit_select by always choosing the first option.
        if params.name.as_ref() == "elicit_select" {
            let args = params.arguments.unwrap_or_default();
            let first = args
                .get("options")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_str())
                .unwrap_or("1")
                .to_string();
            return Ok(CallToolResult::success(vec![Content::text(first)]));
        }
        Err(rmcp::service::ServiceError::Cancelled {
            reason: Some(format!(
                "StubCommunicator does not support tool: {}",
                params.name
            )),
        })
    }

    fn style_context(&self) -> &StyleContext {
        &self.style_ctx
    }

    fn elicitation_context(&self) -> &ElicitationContext {
        &self.elicit_ctx
    }

    fn with_style<T: 'static, S: StyleMarker + elicitation::style::ElicitationStyle + 'static>(
        &self,
        style: S,
    ) -> Self {
        let mut new = self.clone();
        new.style_ctx.set_style::<T, S>(style).ok();
        new
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn make_fighter(name: &str) -> valinoreth::CharacterDescriptor {
    CharacterDescriptorBuilder::default()
        .name(name.to_string())
        .total_points(100)
        .points_spent(100)
        .attributes(vec![])
        .skills(vec![SkillDescriptorBuilder::default()
            .name("Broadsword".to_string())
            .difficulty(SkillDifficulty::Average)
            .base_attribute(AttributeType::DX)
            .points(4)
            .level(12)
            .build()
            .expect("valid skill")])
        .derived_stats(DerivedStatsDescriptor {
            basic_speed: 5.5,
            basic_move: 5,
            dodge: 8,
            hp: 12,
            will: 10,
            perception: 10,
            fp: 12,
        })
        .build()
        .expect("valid character")
}

// ── Workflow smoke tests ───────────────────────────────────────────────────────

/// Run a two-player combat to completion and assert it terminates cleanly.
///
/// Uses [`StubCommunicator`] (always picks option 1) so the test requires no
/// terminal, no LLM, and no API key.
#[tokio::test(flavor = "multi_thread")]
async fn test_workflow_runs_to_completion_with_stub() {
    init_tracing();
    let gm = GameMaster::new();
    let (chat_tx, mut chat_rx) = mpsc::unbounded_channel();

    let players = vec![
        (
            Player::new(make_fighter("Aldric"), StubCommunicator::new()),
            "A".to_string(),
        ),
        (
            Player::new(make_fighter("Mira"), StubCommunicator::new()),
            "B".to_string(),
        ),
    ];

    let mut workflow = CombatWorkflow::new(gm, players, chat_tx);

    // Drain narration in the background so the channel never fills.
    let drain = tokio::spawn(async move {
        let mut msgs = vec![];
        while let Some(msg) = chat_rx.recv().await {
            msgs.push(msg);
        }
        msgs
    });

    let result = tokio::time::timeout(std::time::Duration::from_secs(30), workflow.run())
        .await
        .expect("combat timed out after 30 s");

    match result {
        Ok(Some(winner)) => {
            assert!(!winner.is_empty(), "winner team name should be non-empty");
        }
        Ok(None) => {
            // Draw — both incapacitated simultaneously — also a valid outcome.
        }
        Err(e) => panic!("workflow returned error: {e}"),
    }

    // Drop workflow before awaiting the drain: workflow holds chat_tx, and
    // chat_rx only closes once all senders are dropped.
    drop(workflow);

    let messages = drain.await.expect("drain task panicked");
    assert!(
        !messages.is_empty(),
        "workflow should have emitted at least one chat message"
    );
}

/// Three-player, two-team combat — verifies the workflow handles odd counts.
#[tokio::test(flavor = "multi_thread")]
async fn test_workflow_three_players_two_teams() {
    init_tracing();
    let gm = GameMaster::new();
    let (chat_tx, mut chat_rx) = mpsc::unbounded_channel();

    let players = vec![
        (
            Player::new(make_fighter("Aldric"), StubCommunicator::new()),
            "A".to_string(),
        ),
        (
            Player::new(make_fighter("Mira"), StubCommunicator::new()),
            "B".to_string(),
        ),
        (
            Player::new(make_fighter("Torvald"), StubCommunicator::new()),
            "A".to_string(),
        ),
    ];

    let mut workflow = CombatWorkflow::new(gm, players, chat_tx);

    tokio::spawn(async move { while chat_rx.recv().await.is_some() {} });

    let result = tokio::time::timeout(std::time::Duration::from_secs(30), workflow.run())
        .await
        .expect("combat timed out after 30 s");

    assert!(result.is_ok(), "workflow should not error: {:?}", result);
}

// ── ChatCommunicator channel tests (requires frontend-ratatui) ────────────────

#[cfg(feature = "frontend-ratatui")]
mod chat_communicator_tests {
    use std::sync::Arc;

    use elicitation::ElicitCommunicator;
    use tokio::sync::{mpsc, Mutex};
    use valinoreth::{ChatCommunicator, ChatMessage, ChatSender};

    /// Verify that `send_prompt` posts to the chat log, signals compose mode,
    /// then resolves when `reply_tx` delivers a reply.
    #[tokio::test]
    async fn test_chat_communicator_channel_handoff() {
        let (prompt_tx, mut chat_rx) = mpsc::unbounded_channel::<ChatMessage>();
        let (compose_tx, mut compose_rx) = mpsc::unbounded_channel::<()>();
        let (reply_tx, reply_rx) = mpsc::unbounded_channel::<String>();
        let reply_rx = Arc::new(Mutex::new(reply_rx));

        let comm = ChatCommunicator::new(prompt_tx, compose_tx, reply_rx);

        // Drive send_prompt from a spawned task so we can interact with the
        // channels from this task.
        let send_handle =
            tokio::spawn(async move { comm.send_prompt("Choose: Attack or Defend").await });

        // 1. Prompt should appear in the chat log.
        let msg = chat_rx.recv().await.expect("expected prompt in chat log");
        assert_eq!(msg.sender, ChatSender::GameMaster);
        assert_eq!(msg.text, "Choose: Attack or Defend");

        // 2. Compose signal should have fired.
        compose_rx.recv().await.expect("expected compose signal");

        // 3. Sending a reply unblocks the future.
        reply_tx.send("Attack".to_string()).unwrap();

        let result = send_handle.await.expect("task should not panic");
        assert_eq!(result.expect("should succeed"), "Attack");
    }

    /// Verify that closing `reply_tx` before replying returns an error (the
    /// workflow channel closed — simulates abrupt shutdown).
    #[tokio::test]
    async fn test_chat_communicator_reply_channel_closed() {
        let (prompt_tx, _chat_rx) = mpsc::unbounded_channel::<ChatMessage>();
        let (compose_tx, _compose_rx) = mpsc::unbounded_channel::<()>();
        let (reply_tx, reply_rx) = mpsc::unbounded_channel::<String>();
        let reply_rx = Arc::new(Mutex::new(reply_rx));

        let comm = ChatCommunicator::new(prompt_tx, compose_tx, reply_rx);

        // Drop the sender immediately — the communicator should get an error.
        drop(reply_tx);

        let result = comm.send_prompt("Choose").await;
        assert!(result.is_err(), "closed channel should produce an error");
    }
}
