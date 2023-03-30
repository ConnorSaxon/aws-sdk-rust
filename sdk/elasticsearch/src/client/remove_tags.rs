// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RemoveTags`](crate::client::fluent_builders::RemoveTags) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::RemoveTags::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::RemoveTags::set_arn): <p>Specifies the <code>ARN</code> for the Elasticsearch domain from which you want to delete the specified tags.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::client::fluent_builders::RemoveTags::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::client::fluent_builders::RemoveTags::set_tag_keys): <p>Specifies the <code>TagKey</code> list which you want to remove from the Elasticsearch domain.</p>
                            /// - On success, responds with [`RemoveTagsOutput`](crate::output::RemoveTagsOutput)
                            /// - On failure, responds with [`SdkError<RemoveTagsError>`](crate::error::RemoveTagsError)
    pub fn remove_tags(&self) -> crate::client::fluent_builders::RemoveTags {
                                crate::client::fluent_builders::RemoveTags::new(self.handle.clone())
                            }
}

