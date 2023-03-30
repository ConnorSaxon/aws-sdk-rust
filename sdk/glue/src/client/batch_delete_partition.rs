// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchDeletePartition`](crate::client::fluent_builders::BatchDeletePartition) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::client::fluent_builders::BatchDeletePartition::catalog_id) / [`set_catalog_id(Option<String>)`](crate::client::fluent_builders::BatchDeletePartition::set_catalog_id): <p>The ID of the Data Catalog where the partition to be deleted resides. If none is provided, the Amazon Web Services account ID is used by default.</p>
    ///   - [`database_name(impl Into<String>)`](crate::client::fluent_builders::BatchDeletePartition::database_name) / [`set_database_name(Option<String>)`](crate::client::fluent_builders::BatchDeletePartition::set_database_name): <p>The name of the catalog database in which the table in question resides.</p>
    ///   - [`table_name(impl Into<String>)`](crate::client::fluent_builders::BatchDeletePartition::table_name) / [`set_table_name(Option<String>)`](crate::client::fluent_builders::BatchDeletePartition::set_table_name): <p>The name of the table that contains the partitions to be deleted.</p>
    ///   - [`partitions_to_delete(Vec<PartitionValueList>)`](crate::client::fluent_builders::BatchDeletePartition::partitions_to_delete) / [`set_partitions_to_delete(Option<Vec<PartitionValueList>>)`](crate::client::fluent_builders::BatchDeletePartition::set_partitions_to_delete): <p>A list of <code>PartitionInput</code> structures that define the partitions to be deleted.</p>
                            /// - On success, responds with [`BatchDeletePartitionOutput`](crate::output::BatchDeletePartitionOutput) with field(s):
    ///   - [`errors(Option<Vec<PartitionError>>)`](crate::output::BatchDeletePartitionOutput::errors): <p>The errors encountered when trying to delete the requested partitions.</p>
                            /// - On failure, responds with [`SdkError<BatchDeletePartitionError>`](crate::error::BatchDeletePartitionError)
    pub fn batch_delete_partition(&self) -> crate::client::fluent_builders::BatchDeletePartition {
                                crate::client::fluent_builders::BatchDeletePartition::new(self.handle.clone())
                            }
}

