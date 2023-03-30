// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateTheme`](crate::client::fluent_builders::CreateTheme) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::client::fluent_builders::CreateTheme::app_id) / [`set_app_id(Option<String>)`](crate::client::fluent_builders::CreateTheme::set_app_id): <p>The unique ID of the Amplify app associated with the theme.</p>
    ///   - [`environment_name(impl Into<String>)`](crate::client::fluent_builders::CreateTheme::environment_name) / [`set_environment_name(Option<String>)`](crate::client::fluent_builders::CreateTheme::set_environment_name): <p>The name of the backend environment that is a part of the Amplify app.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateTheme::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateTheme::set_client_token): <p>The unique client token.</p>
    ///   - [`theme_to_create(CreateThemeData)`](crate::client::fluent_builders::CreateTheme::theme_to_create) / [`set_theme_to_create(Option<CreateThemeData>)`](crate::client::fluent_builders::CreateTheme::set_theme_to_create): <p>Represents the configuration of the theme to create.</p>
                            /// - On success, responds with [`CreateThemeOutput`](crate::output::CreateThemeOutput) with field(s):
    ///   - [`entity(Option<Theme>)`](crate::output::CreateThemeOutput::entity): <p>Describes the configuration of the new theme.</p>
                            /// - On failure, responds with [`SdkError<CreateThemeError>`](crate::error::CreateThemeError)
    pub fn create_theme(&self) -> crate::client::fluent_builders::CreateTheme {
                                crate::client::fluent_builders::CreateTheme::new(self.handle.clone())
                            }
}

