// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteTags`](crate::client::fluent_builders::DeleteTags) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DeleteTags::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DeleteTags::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`resources(Vec<String>)`](crate::client::fluent_builders::DeleteTags::resources) / [`set_resources(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteTags::set_resources): <p>The IDs of the resources, separated by spaces.</p>  <p>Constraints: Up to 1000 resource IDs. We recommend breaking up this request into smaller batches.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::DeleteTags::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::DeleteTags::set_tags): <p>The tags to delete. Specify a tag key and an optional tag value to delete specific tags. If you specify a tag key without a tag value, we delete any tag with this key regardless of its value. If you specify a tag key with an empty string as the tag value, we delete the tag only if its value is an empty string.</p>  <p>If you omit this parameter, we delete all user-defined tags for the specified resources. We do not delete Amazon Web Services-generated tags (tags that have the <code>aws:</code> prefix).</p>  <p>Constraints: Up to 1000 tags.</p>
                            /// - On success, responds with [`DeleteTagsOutput`](crate::output::DeleteTagsOutput)
                            /// - On failure, responds with [`SdkError<DeleteTagsError>`](crate::error::DeleteTagsError)
    pub fn delete_tags(&self) -> crate::client::fluent_builders::DeleteTags {
                                crate::client::fluent_builders::DeleteTags::new(self.handle.clone())
                            }
}

