// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteTags`](crate::client::fluent_builders::DeleteTags) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_ids(Vec<String>)`](crate::client::fluent_builders::DeleteTags::configuration_ids) / [`set_configuration_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteTags::set_configuration_ids): <p>A list of configuration items with tags that you want to delete.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::DeleteTags::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::DeleteTags::set_tags): <p>Tags that you want to delete from one or more configuration items. Specify the tags that you want to delete in a <i>key</i>-<i>value</i> format. For example:</p>  <p> <code>{"key": "serverType", "value": "webServer"}</code> </p>
                            /// - On success, responds with [`DeleteTagsOutput`](crate::output::DeleteTagsOutput)
                            /// - On failure, responds with [`SdkError<DeleteTagsError>`](crate::error::DeleteTagsError)
    pub fn delete_tags(&self) -> crate::client::fluent_builders::DeleteTags {
                                crate::client::fluent_builders::DeleteTags::new(self.handle.clone())
                            }
}

