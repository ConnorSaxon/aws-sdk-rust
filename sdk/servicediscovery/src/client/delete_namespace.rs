// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteNamespace`](crate::client::fluent_builders::DeleteNamespace) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteNamespace::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteNamespace::set_id): <p>The ID of the namespace that you want to delete.</p>
                            /// - On success, responds with [`DeleteNamespaceOutput`](crate::output::DeleteNamespaceOutput) with field(s):
    ///   - [`operation_id(Option<String>)`](crate::output::DeleteNamespaceOutput::operation_id): <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
                            /// - On failure, responds with [`SdkError<DeleteNamespaceError>`](crate::error::DeleteNamespaceError)
    pub fn delete_namespace(&self) -> crate::client::fluent_builders::DeleteNamespace {
                                crate::client::fluent_builders::DeleteNamespace::new(self.handle.clone())
                            }
}

