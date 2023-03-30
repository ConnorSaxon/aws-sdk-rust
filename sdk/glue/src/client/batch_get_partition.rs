// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchGetPartition`](crate::client::fluent_builders::BatchGetPartition) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::client::fluent_builders::BatchGetPartition::catalog_id) / [`set_catalog_id(Option<String>)`](crate::client::fluent_builders::BatchGetPartition::set_catalog_id): <p>The ID of the Data Catalog where the partitions in question reside. If none is supplied, the Amazon Web Services account ID is used by default.</p>
    ///   - [`database_name(impl Into<String>)`](crate::client::fluent_builders::BatchGetPartition::database_name) / [`set_database_name(Option<String>)`](crate::client::fluent_builders::BatchGetPartition::set_database_name): <p>The name of the catalog database where the partitions reside.</p>
    ///   - [`table_name(impl Into<String>)`](crate::client::fluent_builders::BatchGetPartition::table_name) / [`set_table_name(Option<String>)`](crate::client::fluent_builders::BatchGetPartition::set_table_name): <p>The name of the partitions' table.</p>
    ///   - [`partitions_to_get(Vec<PartitionValueList>)`](crate::client::fluent_builders::BatchGetPartition::partitions_to_get) / [`set_partitions_to_get(Option<Vec<PartitionValueList>>)`](crate::client::fluent_builders::BatchGetPartition::set_partitions_to_get): <p>A list of partition values identifying the partitions to retrieve.</p>
                            /// - On success, responds with [`BatchGetPartitionOutput`](crate::output::BatchGetPartitionOutput) with field(s):
    ///   - [`partitions(Option<Vec<Partition>>)`](crate::output::BatchGetPartitionOutput::partitions): <p>A list of the requested partitions.</p>
    ///   - [`unprocessed_keys(Option<Vec<PartitionValueList>>)`](crate::output::BatchGetPartitionOutput::unprocessed_keys): <p>A list of the partition values in the request for which partitions were not returned.</p>
                            /// - On failure, responds with [`SdkError<BatchGetPartitionError>`](crate::error::BatchGetPartitionError)
    pub fn batch_get_partition(&self) -> crate::client::fluent_builders::BatchGetPartition {
                                crate::client::fluent_builders::BatchGetPartition::new(self.handle.clone())
                            }
}

