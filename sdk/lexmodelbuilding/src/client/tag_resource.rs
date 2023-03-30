// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`TagResource`](crate::client::fluent_builders::TagResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::TagResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::TagResource::set_resource_arn): <p>The Amazon Resource Name (ARN) of the bot, bot alias, or bot channel to tag.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::TagResource::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::TagResource::set_tags): <p>A list of tag keys to add to the resource. If a tag key already exists, the existing value is replaced with the new value.</p>
                            /// - On success, responds with [`TagResourceOutput`](crate::output::TagResourceOutput)
                            /// - On failure, responds with [`SdkError<TagResourceError>`](crate::error::TagResourceError)
    pub fn tag_resource(&self) -> crate::client::fluent_builders::TagResource {
                                crate::client::fluent_builders::TagResource::new(self.handle.clone())
                            }
}

