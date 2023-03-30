// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetModelTemplate`](crate::client::fluent_builders::GetModelTemplate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl Into<String>)`](crate::client::fluent_builders::GetModelTemplate::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::client::fluent_builders::GetModelTemplate::set_rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`model_name(impl Into<String>)`](crate::client::fluent_builders::GetModelTemplate::model_name) / [`set_model_name(Option<String>)`](crate::client::fluent_builders::GetModelTemplate::set_model_name): <p>The name of the model for which to generate a template.</p>
                            /// - On success, responds with [`GetModelTemplateOutput`](crate::output::GetModelTemplateOutput) with field(s):
    ///   - [`value(Option<String>)`](crate::output::GetModelTemplateOutput::value): <p>The Apache Velocity Template Language (VTL) template content used for the template resource.</p>
                            /// - On failure, responds with [`SdkError<GetModelTemplateError>`](crate::error::GetModelTemplateError)
    pub fn get_model_template(&self) -> crate::client::fluent_builders::GetModelTemplate {
                                crate::client::fluent_builders::GetModelTemplate::new(self.handle.clone())
                            }
}

