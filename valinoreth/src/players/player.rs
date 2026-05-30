//! [`Player`] — a character in an active game session, driven by a communicator.
//!
//! A `Player` pairs a [`CharacterDescriptor`] (the GURPS stats) with an
//! [`ElicitCommunicator`] (who is driving decisions).  The communicator is the
//! only thing that distinguishes a human player from an agent — the game logic
//! is identical regardless.
//!
//! ## Decision flow
//!
//! ```text
//! GameMaster calls player.choose_maneuver()
//!   → ManeuverChoice::elicit(&self.communicator)
//!     → TuiCommunicator: render numbered list, read keystroke
//!     → LlmElicitCommunicator: send prompt to LLM, parse response
//!   ← ManeuverChoice variant
//! ```

use elicitation::{ChoiceSet, ElicitCommunicator, ElicitResult, Elicitation as _};
use tracing::instrument;

use crate::{CharacterDescriptor, DefenseChoice, ManeuverChoice};

/// A character in an active game session paired with its decision driver.
///
/// Generic over `C: ElicitCommunicator` so the same type works for human
/// players ([`TuiCommunicator`]) and agent players (`LlmElicitCommunicator`).
///
/// [`TuiCommunicator`]: crate::TuiCommunicator
pub struct Player<C: ElicitCommunicator> {
    /// The GURPS character controlled by this player.
    pub character: CharacterDescriptor,
    /// The communicator that drives decision elicitation.
    pub communicator: C,
}

impl<C: ElicitCommunicator> Player<C> {
    /// Create a new player from a character and a communicator.
    #[instrument(skip(character, communicator), fields(name = %character.name))]
    pub fn new(character: CharacterDescriptor, communicator: C) -> Self {
        Self {
            character,
            communicator,
        }
    }

    /// Elicit a maneuver choice from this player for their combat turn.
    ///
    /// Presents the available maneuvers and blocks until the player (or agent)
    /// selects one.
    ///
    /// # Errors
    ///
    /// Returns an error if the communicator fails (I/O error, Ctrl-C, LLM
    /// failure, etc.).
    #[instrument(skip(self), fields(character_id = %self.character.name))]
    pub async fn choose_maneuver(&self) -> ElicitResult<ManeuverChoice> {
        ManeuverChoice::elicit(&self.communicator).await
    }

    /// Elicit a defense choice from this player in response to an incoming attack.
    ///
    /// `available` is a [`ChoiceSet`] of the defenses that are actually usable
    /// given the character's current state (weapon readiness, shield equipped,
    /// All-Out Attack penalty, etc.).  The caller filters the options; this
    /// method only handles elicitation.
    ///
    /// # Errors
    ///
    /// Returns an error if the communicator fails or the choice set is empty.
    #[instrument(skip(self, available), fields(character_id = %self.character.name))]
    pub async fn choose_defense(
        &self,
        available: ChoiceSet<DefenseChoice>,
    ) -> ElicitResult<DefenseChoice> {
        available.elicit(&self.communicator).await
    }
}
