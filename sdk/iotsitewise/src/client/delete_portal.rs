// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeletePortal`](crate::client::fluent_builders::DeletePortal) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`portal_id(impl Into<String>)`](crate::client::fluent_builders::DeletePortal::portal_id) / [`set_portal_id(Option<String>)`](crate::client::fluent_builders::DeletePortal::set_portal_id): <p>The ID of the portal to delete.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::DeletePortal::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::DeletePortal::set_client_token): <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
                            /// - On success, responds with [`DeletePortalOutput`](crate::output::DeletePortalOutput) with field(s):
    ///   - [`portal_status(Option<PortalStatus>)`](crate::output::DeletePortalOutput::portal_status): <p>The status of the portal, which contains a state (<code>DELETING</code> after successfully calling this operation) and any error message.</p>
                            /// - On failure, responds with [`SdkError<DeletePortalError>`](crate::error::DeletePortalError)
    pub fn delete_portal(&self) -> crate::client::fluent_builders::DeletePortal {
                                crate::client::fluent_builders::DeletePortal::new(self.handle.clone())
                            }
}

