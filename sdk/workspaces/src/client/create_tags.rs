// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateTags`](crate::client::fluent_builders::CreateTags) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_id(impl Into<String>)`](crate::client::fluent_builders::CreateTags::resource_id) / [`set_resource_id(Option<String>)`](crate::client::fluent_builders::CreateTags::set_resource_id): <p>The identifier of the WorkSpaces resource. The supported resource types are WorkSpaces, registered directories, images, custom bundles, IP access control groups, and connection aliases.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateTags::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateTags::set_tags): <p>The tags. Each WorkSpaces resource can have a maximum of 50 tags.</p>
                            /// - On success, responds with [`CreateTagsOutput`](crate::output::CreateTagsOutput)
                            /// - On failure, responds with [`SdkError<CreateTagsError>`](crate::error::CreateTagsError)
    pub fn create_tags(&self) -> crate::client::fluent_builders::CreateTags {
                                crate::client::fluent_builders::CreateTags::new(self.handle.clone())
                            }
}

