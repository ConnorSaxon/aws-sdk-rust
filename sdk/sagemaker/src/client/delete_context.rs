// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteContext`](crate::client::fluent_builders::DeleteContext) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`context_name(impl Into<String>)`](crate::client::fluent_builders::DeleteContext::context_name) / [`set_context_name(Option<String>)`](crate::client::fluent_builders::DeleteContext::set_context_name): <p>The name of the context to delete.</p>
                            /// - On success, responds with [`DeleteContextOutput`](crate::output::DeleteContextOutput) with field(s):
    ///   - [`context_arn(Option<String>)`](crate::output::DeleteContextOutput::context_arn): <p>The Amazon Resource Name (ARN) of the context.</p>
                            /// - On failure, responds with [`SdkError<DeleteContextError>`](crate::error::DeleteContextError)
    pub fn delete_context(&self) -> crate::client::fluent_builders::DeleteContext {
                                crate::client::fluent_builders::DeleteContext::new(self.handle.clone())
                            }
}

