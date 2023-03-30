// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetGame`](crate::client::fluent_builders::GetGame) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`game_name(impl Into<String>)`](crate::client::fluent_builders::GetGame::game_name) / [`set_game_name(Option<String>)`](crate::client::fluent_builders::GetGame::set_game_name): <p>The name of the game.</p>
                            /// - On success, responds with [`GetGameOutput`](crate::output::GetGameOutput) with field(s):
    ///   - [`game(Option<GameDetails>)`](crate::output::GetGameOutput::game): <p>The details of the game.</p>
                            /// - On failure, responds with [`SdkError<GetGameError>`](crate::error::GetGameError)
    pub fn get_game(&self) -> crate::client::fluent_builders::GetGame {
                                crate::client::fluent_builders::GetGame::new(self.handle.clone())
                            }
}

