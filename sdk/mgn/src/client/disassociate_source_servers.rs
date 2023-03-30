// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateSourceServers`](crate::client::fluent_builders::DisassociateSourceServers) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateSourceServers::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::DisassociateSourceServers::set_application_id): <p>Application ID.</p>
    ///   - [`source_server_i_ds(Vec<String>)`](crate::client::fluent_builders::DisassociateSourceServers::source_server_i_ds) / [`set_source_server_i_ds(Option<Vec<String>>)`](crate::client::fluent_builders::DisassociateSourceServers::set_source_server_i_ds): <p>Source server IDs list.</p>
                            /// - On success, responds with [`DisassociateSourceServersOutput`](crate::output::DisassociateSourceServersOutput)
                            /// - On failure, responds with [`SdkError<DisassociateSourceServersError>`](crate::error::DisassociateSourceServersError)
    pub fn disassociate_source_servers(&self) -> crate::client::fluent_builders::DisassociateSourceServers {
                                crate::client::fluent_builders::DisassociateSourceServers::new(self.handle.clone())
                            }
}

