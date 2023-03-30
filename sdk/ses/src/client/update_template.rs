// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateTemplate`](crate::client::fluent_builders::UpdateTemplate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`template(Template)`](crate::client::fluent_builders::UpdateTemplate::template) / [`set_template(Option<Template>)`](crate::client::fluent_builders::UpdateTemplate::set_template): <p>The content of the email, composed of a subject line, an HTML part, and a text-only part.</p>
                            /// - On success, responds with [`UpdateTemplateOutput`](crate::output::UpdateTemplateOutput)
                            /// - On failure, responds with [`SdkError<UpdateTemplateError>`](crate::error::UpdateTemplateError)
    pub fn update_template(&self) -> crate::client::fluent_builders::UpdateTemplate {
                                crate::client::fluent_builders::UpdateTemplate::new(self.handle.clone())
                            }
}

