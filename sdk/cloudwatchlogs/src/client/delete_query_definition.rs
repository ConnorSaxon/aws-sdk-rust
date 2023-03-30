// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteQueryDefinition`](crate::client::fluent_builders::DeleteQueryDefinition) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`query_definition_id(impl Into<String>)`](crate::client::fluent_builders::DeleteQueryDefinition::query_definition_id) / [`set_query_definition_id(Option<String>)`](crate::client::fluent_builders::DeleteQueryDefinition::set_query_definition_id): <p>The ID of the query definition that you want to delete. You can use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a> to retrieve the IDs of your saved query definitions.</p>
                            /// - On success, responds with [`DeleteQueryDefinitionOutput`](crate::output::DeleteQueryDefinitionOutput) with field(s):
    ///   - [`success(bool)`](crate::output::DeleteQueryDefinitionOutput::success): <p>A value of TRUE indicates that the operation succeeded. FALSE indicates that the operation failed.</p>
                            /// - On failure, responds with [`SdkError<DeleteQueryDefinitionError>`](crate::error::DeleteQueryDefinitionError)
    pub fn delete_query_definition(&self) -> crate::client::fluent_builders::DeleteQueryDefinition {
                                crate::client::fluent_builders::DeleteQueryDefinition::new(self.handle.clone())
                            }
}

