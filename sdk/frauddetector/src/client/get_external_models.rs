// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetExternalModels`](crate::client::fluent_builders::GetExternalModels) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetExternalModels::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`model_endpoint(impl Into<String>)`](crate::client::fluent_builders::GetExternalModels::model_endpoint) / [`set_model_endpoint(Option<String>)`](crate::client::fluent_builders::GetExternalModels::set_model_endpoint): <p>The Amazon SageMaker model endpoint.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetExternalModels::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetExternalModels::set_next_token): <p>The next page token for the request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetExternalModels::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetExternalModels::set_max_results): <p>The maximum number of objects to return for the request.</p>
                            /// - On success, responds with [`GetExternalModelsOutput`](crate::output::GetExternalModelsOutput) with field(s):
    ///   - [`external_models(Option<Vec<ExternalModel>>)`](crate::output::GetExternalModelsOutput::external_models): <p>Gets the Amazon SageMaker models.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetExternalModelsOutput::next_token): <p>The next page token to be used in subsequent requests.</p>
                            /// - On failure, responds with [`SdkError<GetExternalModelsError>`](crate::error::GetExternalModelsError)
    pub fn get_external_models(&self) -> crate::client::fluent_builders::GetExternalModels {
                                crate::client::fluent_builders::GetExternalModels::new(self.handle.clone())
                            }
}

