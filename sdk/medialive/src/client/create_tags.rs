// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateTags`](crate::client::fluent_builders::CreateTags) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::CreateTags::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::CreateTags::set_resource_arn): Placeholder documentation for __string
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateTags::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateTags::set_tags): Placeholder documentation for Tags
                            /// - On success, responds with [`CreateTagsOutput`](crate::output::CreateTagsOutput)
                            /// - On failure, responds with [`SdkError<CreateTagsError>`](crate::error::CreateTagsError)
    pub fn create_tags(&self) -> crate::client::fluent_builders::CreateTags {
                                crate::client::fluent_builders::CreateTags::new(self.handle.clone())
                            }
}

