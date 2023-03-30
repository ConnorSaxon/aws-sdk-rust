// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AddTags`](crate::client::fluent_builders::AddTags) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::AddTags::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::AddTags::set_arn): <p> Specify the <code>ARN</code> for which you want to add the tags.</p>
    ///   - [`tag_list(Vec<Tag>)`](crate::client::fluent_builders::AddTags::tag_list) / [`set_tag_list(Option<Vec<Tag>>)`](crate::client::fluent_builders::AddTags::set_tag_list): <p> List of <code>Tag</code> that need to be added for the Elasticsearch domain. </p>
                            /// - On success, responds with [`AddTagsOutput`](crate::output::AddTagsOutput)
                            /// - On failure, responds with [`SdkError<AddTagsError>`](crate::error::AddTagsError)
    pub fn add_tags(&self) -> crate::client::fluent_builders::AddTags {
                                crate::client::fluent_builders::AddTags::new(self.handle.clone())
                            }
}

