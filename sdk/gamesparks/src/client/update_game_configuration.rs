// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateGameConfiguration`](crate::client::fluent_builders::UpdateGameConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`game_name(impl Into<String>)`](crate::client::fluent_builders::UpdateGameConfiguration::game_name) / [`set_game_name(Option<String>)`](crate::client::fluent_builders::UpdateGameConfiguration::set_game_name): <p>The name of the game.</p>
    ///   - [`modifications(Vec<SectionModification>)`](crate::client::fluent_builders::UpdateGameConfiguration::modifications) / [`set_modifications(Option<Vec<SectionModification>>)`](crate::client::fluent_builders::UpdateGameConfiguration::set_modifications): <p>The list of modifications to make.</p>
                            /// - On success, responds with [`UpdateGameConfigurationOutput`](crate::output::UpdateGameConfigurationOutput) with field(s):
    ///   - [`game_configuration(Option<GameConfigurationDetails>)`](crate::output::UpdateGameConfigurationOutput::game_configuration): <p>Details about the game configuration.</p>
                            /// - On failure, responds with [`SdkError<UpdateGameConfigurationError>`](crate::error::UpdateGameConfigurationError)
    pub fn update_game_configuration(&self) -> crate::client::fluent_builders::UpdateGameConfiguration {
                                crate::client::fluent_builders::UpdateGameConfiguration::new(self.handle.clone())
                            }
}

