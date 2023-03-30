// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteApi`](crate::client::fluent_builders::DeleteApi) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`api_id(impl Into<String>)`](crate::client::fluent_builders::DeleteApi::api_id) / [`set_api_id(Option<String>)`](crate::client::fluent_builders::DeleteApi::set_api_id): <p>The API identifier.</p>
                            /// - On success, responds with [`DeleteApiOutput`](crate::output::DeleteApiOutput)
                            /// - On failure, responds with [`SdkError<DeleteApiError>`](crate::error::DeleteApiError)
    pub fn delete_api(&self) -> crate::client::fluent_builders::DeleteApi {
                                crate::client::fluent_builders::DeleteApi::new(self.handle.clone())
                            }
}

