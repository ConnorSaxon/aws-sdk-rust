// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::client::fluent_builders::ListTagsForResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_marker(impl Into<String>)`](crate::client::fluent_builders::ListTagsForResource::next_marker) / [`set_next_marker(Option<String>)`](crate::client::fluent_builders::ListTagsForResource::set_next_marker): <p></p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListTagsForResource::limit) / [`set_limit(i32)`](crate::client::fluent_builders::ListTagsForResource::set_limit): <p></p>
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::ListTagsForResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::ListTagsForResource::set_resource_arn): <p></p>
                            /// - On success, responds with [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::output::ListTagsForResourceOutput::next_marker): <p></p>
    ///   - [`tag_info_for_resource(Option<TagInfoForResource>)`](crate::output::ListTagsForResourceOutput::tag_info_for_resource): <p></p>
                            /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::error::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::client::fluent_builders::ListTagsForResource {
                                crate::client::fluent_builders::ListTagsForResource::new(self.handle.clone())
                            }
}

