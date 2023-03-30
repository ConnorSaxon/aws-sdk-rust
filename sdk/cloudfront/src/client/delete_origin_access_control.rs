// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteOriginAccessControl`](crate::client::fluent_builders::DeleteOriginAccessControl) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteOriginAccessControl::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteOriginAccessControl::set_id): <p>The unique identifier of the origin access control that you are deleting.</p>
    ///   - [`if_match(impl Into<String>)`](crate::client::fluent_builders::DeleteOriginAccessControl::if_match) / [`set_if_match(Option<String>)`](crate::client::fluent_builders::DeleteOriginAccessControl::set_if_match): <p>The current version (<code>ETag</code> value) of the origin access control that you are deleting.</p>
                            /// - On success, responds with [`DeleteOriginAccessControlOutput`](crate::output::DeleteOriginAccessControlOutput)
                            /// - On failure, responds with [`SdkError<DeleteOriginAccessControlError>`](crate::error::DeleteOriginAccessControlError)
    pub fn delete_origin_access_control(&self) -> crate::client::fluent_builders::DeleteOriginAccessControl {
                                crate::client::fluent_builders::DeleteOriginAccessControl::new(self.handle.clone())
                            }
}

