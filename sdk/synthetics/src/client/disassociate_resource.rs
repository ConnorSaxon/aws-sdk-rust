// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateResource`](crate::client::fluent_builders::DisassociateResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`group_identifier(impl Into<String>)`](crate::client::fluent_builders::DisassociateResource::group_identifier) / [`set_group_identifier(Option<String>)`](crate::client::fluent_builders::DisassociateResource::set_group_identifier): <p>Specifies the group. You can specify the group name, the ARN, or the group ID as the <code>GroupIdentifier</code>.</p>
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::DisassociateResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::DisassociateResource::set_resource_arn): <p>The ARN of the canary that you want to remove from the specified group.</p>
                            /// - On success, responds with [`DisassociateResourceOutput`](crate::output::DisassociateResourceOutput)
                            /// - On failure, responds with [`SdkError<DisassociateResourceError>`](crate::error::DisassociateResourceError)
    pub fn disassociate_resource(&self) -> crate::client::fluent_builders::DisassociateResource {
                                crate::client::fluent_builders::DisassociateResource::new(self.handle.clone())
                            }
}

