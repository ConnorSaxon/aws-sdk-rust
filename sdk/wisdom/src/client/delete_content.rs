// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteContent`](crate::client::fluent_builders::DeleteContent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`knowledge_base_id(impl Into<String>)`](crate::client::fluent_builders::DeleteContent::knowledge_base_id) / [`set_knowledge_base_id(Option<String>)`](crate::client::fluent_builders::DeleteContent::set_knowledge_base_id): <p>The identifier of the knowledge base. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    ///   - [`content_id(impl Into<String>)`](crate::client::fluent_builders::DeleteContent::content_id) / [`set_content_id(Option<String>)`](crate::client::fluent_builders::DeleteContent::set_content_id): <p>The identifier of the content. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
                            /// - On success, responds with [`DeleteContentOutput`](crate::output::DeleteContentOutput)
                            /// - On failure, responds with [`SdkError<DeleteContentError>`](crate::error::DeleteContentError)
    pub fn delete_content(&self) -> crate::client::fluent_builders::DeleteContent {
                                crate::client::fluent_builders::DeleteContent::new(self.handle.clone())
                            }
}

