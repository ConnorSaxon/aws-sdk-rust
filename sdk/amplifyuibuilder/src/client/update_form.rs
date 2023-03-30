// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateForm`](crate::client::fluent_builders::UpdateForm) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::client::fluent_builders::UpdateForm::app_id) / [`set_app_id(Option<String>)`](crate::client::fluent_builders::UpdateForm::set_app_id): <p>The unique ID for the Amplify app.</p>
    ///   - [`environment_name(impl Into<String>)`](crate::client::fluent_builders::UpdateForm::environment_name) / [`set_environment_name(Option<String>)`](crate::client::fluent_builders::UpdateForm::set_environment_name): <p>The name of the backend environment that is part of the Amplify app.</p>
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::UpdateForm::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::UpdateForm::set_id): <p>The unique ID for the form.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::UpdateForm::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::UpdateForm::set_client_token): <p>The unique client token.</p>
    ///   - [`updated_form(UpdateFormData)`](crate::client::fluent_builders::UpdateForm::updated_form) / [`set_updated_form(Option<UpdateFormData>)`](crate::client::fluent_builders::UpdateForm::set_updated_form): <p>The request accepts the following data in JSON format.</p>
                            /// - On success, responds with [`UpdateFormOutput`](crate::output::UpdateFormOutput) with field(s):
    ///   - [`entity(Option<Form>)`](crate::output::UpdateFormOutput::entity): <p>Describes the configuration of the updated form.</p>
                            /// - On failure, responds with [`SdkError<UpdateFormError>`](crate::error::UpdateFormError)
    pub fn update_form(&self) -> crate::client::fluent_builders::UpdateForm {
                                crate::client::fluent_builders::UpdateForm::new(self.handle.clone())
                            }
}

