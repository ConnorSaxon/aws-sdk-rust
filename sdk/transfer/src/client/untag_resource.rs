// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UntagResource`](crate::client::fluent_builders::UntagResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::UntagResource::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::UntagResource::set_arn): <p>The value of the resource that will have the tag removed. An Amazon Resource Name (ARN) is an identifier for a specific Amazon Web Services resource, such as a server, user, or role.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::client::fluent_builders::UntagResource::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UntagResource::set_tag_keys): <p>TagKeys are key-value pairs assigned to ARNs that can be used to group and search for resources by type. This metadata can be attached to resources for any purpose.</p>
                            /// - On success, responds with [`UntagResourceOutput`](crate::output::UntagResourceOutput)
                            /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::error::UntagResourceError)
    pub fn untag_resource(&self) -> crate::client::fluent_builders::UntagResource {
                                crate::client::fluent_builders::UntagResource::new(self.handle.clone())
                            }
}

