// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UntagResource`](crate::client::fluent_builders::UntagResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::UntagResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::UntagResource::set_resource_arn): <p>The ARN of the resource to remove the tag from.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::client::fluent_builders::UntagResource::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UntagResource::set_tag_keys): <p>The key for the tag to add.</p>
                            /// - On success, responds with [`UntagResourceOutput`](crate::output::UntagResourceOutput)
                            /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::error::UntagResourceError)
    pub fn untag_resource(&self) -> crate::client::fluent_builders::UntagResource {
                                crate::client::fluent_builders::UntagResource::new(self.handle.clone())
                            }
}

