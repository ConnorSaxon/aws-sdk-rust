// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteGraph`](crate::client::fluent_builders::DeleteGraph) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`graph_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteGraph::graph_arn) / [`set_graph_arn(Option<String>)`](crate::client::fluent_builders::DeleteGraph::set_graph_arn): <p>The ARN of the behavior graph to disable.</p>
                            /// - On success, responds with [`DeleteGraphOutput`](crate::output::DeleteGraphOutput)
                            /// - On failure, responds with [`SdkError<DeleteGraphError>`](crate::error::DeleteGraphError)
    pub fn delete_graph(&self) -> crate::client::fluent_builders::DeleteGraph {
                                crate::client::fluent_builders::DeleteGraph::new(self.handle.clone())
                            }
}

