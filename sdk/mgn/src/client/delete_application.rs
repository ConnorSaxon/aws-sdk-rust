// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteApplication`](crate::client::fluent_builders::DeleteApplication) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::DeleteApplication::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::DeleteApplication::set_application_id): <p>Application ID.</p>
                            /// - On success, responds with [`DeleteApplicationOutput`](crate::output::DeleteApplicationOutput)
                            /// - On failure, responds with [`SdkError<DeleteApplicationError>`](crate::error::DeleteApplicationError)
    pub fn delete_application(&self) -> crate::client::fluent_builders::DeleteApplication {
                                crate::client::fluent_builders::DeleteApplication::new(self.handle.clone())
                            }
}

