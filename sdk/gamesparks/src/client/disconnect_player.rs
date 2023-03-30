// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisconnectPlayer`](crate::client::fluent_builders::DisconnectPlayer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`player_id(impl Into<String>)`](crate::client::fluent_builders::DisconnectPlayer::player_id) / [`set_player_id(Option<String>)`](crate::client::fluent_builders::DisconnectPlayer::set_player_id): <p>The unique identifier representing a player.</p>
    ///   - [`game_name(impl Into<String>)`](crate::client::fluent_builders::DisconnectPlayer::game_name) / [`set_game_name(Option<String>)`](crate::client::fluent_builders::DisconnectPlayer::set_game_name): <p>The name of the game.</p>
    ///   - [`stage_name(impl Into<String>)`](crate::client::fluent_builders::DisconnectPlayer::stage_name) / [`set_stage_name(Option<String>)`](crate::client::fluent_builders::DisconnectPlayer::set_stage_name): <p>The name of the stage.</p>
                            /// - On success, responds with [`DisconnectPlayerOutput`](crate::output::DisconnectPlayerOutput) with field(s):
    ///   - [`disconnect_successes(Option<Vec<String>>)`](crate::output::DisconnectPlayerOutput::disconnect_successes): <p>The list of the connection ids that were disconnected.</p>
    ///   - [`disconnect_failures(Option<Vec<String>>)`](crate::output::DisconnectPlayerOutput::disconnect_failures): <p>The list of the connection ids that could not be disconnected.</p>
                            /// - On failure, responds with [`SdkError<DisconnectPlayerError>`](crate::error::DisconnectPlayerError)
    pub fn disconnect_player(&self) -> crate::client::fluent_builders::DisconnectPlayer {
                                crate::client::fluent_builders::DisconnectPlayer::new(self.handle.clone())
                            }
}

