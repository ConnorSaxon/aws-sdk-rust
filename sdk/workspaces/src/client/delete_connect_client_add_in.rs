// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteConnectClientAddIn`](crate::client::fluent_builders::DeleteConnectClientAddIn) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`add_in_id(impl Into<String>)`](crate::client::fluent_builders::DeleteConnectClientAddIn::add_in_id) / [`set_add_in_id(Option<String>)`](crate::client::fluent_builders::DeleteConnectClientAddIn::set_add_in_id): <p>The identifier of the client add-in to delete.</p>
    ///   - [`resource_id(impl Into<String>)`](crate::client::fluent_builders::DeleteConnectClientAddIn::resource_id) / [`set_resource_id(Option<String>)`](crate::client::fluent_builders::DeleteConnectClientAddIn::set_resource_id): <p>The directory identifier for which the client add-in is configured.</p>
                            /// - On success, responds with [`DeleteConnectClientAddInOutput`](crate::output::DeleteConnectClientAddInOutput)
                            /// - On failure, responds with [`SdkError<DeleteConnectClientAddInError>`](crate::error::DeleteConnectClientAddInError)
    pub fn delete_connect_client_add_in(&self) -> crate::client::fluent_builders::DeleteConnectClientAddIn {
                                crate::client::fluent_builders::DeleteConnectClientAddIn::new(self.handle.clone())
                            }
}

