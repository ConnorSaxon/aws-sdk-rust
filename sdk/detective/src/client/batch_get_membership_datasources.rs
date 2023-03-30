// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchGetMembershipDatasources`](crate::client::fluent_builders::BatchGetMembershipDatasources) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`graph_arns(Vec<String>)`](crate::client::fluent_builders::BatchGetMembershipDatasources::graph_arns) / [`set_graph_arns(Option<Vec<String>>)`](crate::client::fluent_builders::BatchGetMembershipDatasources::set_graph_arns): <p>The ARN of the behavior graph.</p>
                            /// - On success, responds with [`BatchGetMembershipDatasourcesOutput`](crate::output::BatchGetMembershipDatasourcesOutput) with field(s):
    ///   - [`membership_datasources(Option<Vec<MembershipDatasources>>)`](crate::output::BatchGetMembershipDatasourcesOutput::membership_datasources): <p>Details on the data source package history for an member of the behavior graph.</p>
    ///   - [`unprocessed_graphs(Option<Vec<UnprocessedGraph>>)`](crate::output::BatchGetMembershipDatasourcesOutput::unprocessed_graphs): <p>Graphs that data source package information could not be retrieved for.</p>
                            /// - On failure, responds with [`SdkError<BatchGetMembershipDatasourcesError>`](crate::error::BatchGetMembershipDatasourcesError)
    pub fn batch_get_membership_datasources(&self) -> crate::client::fluent_builders::BatchGetMembershipDatasources {
                                crate::client::fluent_builders::BatchGetMembershipDatasources::new(self.handle.clone())
                            }
}

