// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UntagResource`](crate::client::fluent_builders::UntagResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::UntagResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::UntagResource::set_resource_arn): <p>Specifies the Amazon Resource Name (ARN) of the resource to remove the tags from.</p>
    ///   - [`keys(Vec<String>)`](crate::client::fluent_builders::UntagResource::keys) / [`set_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UntagResource::set_keys): <p>Specifies the keys in the tags that you want to remove.</p>
                            /// - On success, responds with [`UntagResourceOutput`](crate::output::UntagResourceOutput)
                            /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::error::UntagResourceError)
    pub fn untag_resource(&self) -> crate::client::fluent_builders::UntagResource {
                                crate::client::fluent_builders::UntagResource::new(self.handle.clone())
                            }
}

