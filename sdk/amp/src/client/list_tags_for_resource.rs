// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::client::fluent_builders::ListTagsForResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::ListTagsForResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::ListTagsForResource::set_resource_arn): The ARN of the resource.
                            /// - On success, responds with [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput) with field(s):
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::ListTagsForResourceOutput::tags): The list of tags assigned to the resource.
                            /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::error::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::client::fluent_builders::ListTagsForResource {
                                crate::client::fluent_builders::ListTagsForResource::new(self.handle.clone())
                            }
}

