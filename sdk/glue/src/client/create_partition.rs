// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreatePartition`](crate::client::fluent_builders::CreatePartition) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::client::fluent_builders::CreatePartition::catalog_id) / [`set_catalog_id(Option<String>)`](crate::client::fluent_builders::CreatePartition::set_catalog_id): <p>The Amazon Web Services account ID of the catalog in which the partition is to be created.</p>
    ///   - [`database_name(impl Into<String>)`](crate::client::fluent_builders::CreatePartition::database_name) / [`set_database_name(Option<String>)`](crate::client::fluent_builders::CreatePartition::set_database_name): <p>The name of the metadata database in which the partition is to be created.</p>
    ///   - [`table_name(impl Into<String>)`](crate::client::fluent_builders::CreatePartition::table_name) / [`set_table_name(Option<String>)`](crate::client::fluent_builders::CreatePartition::set_table_name): <p>The name of the metadata table in which the partition is to be created.</p>
    ///   - [`partition_input(PartitionInput)`](crate::client::fluent_builders::CreatePartition::partition_input) / [`set_partition_input(Option<PartitionInput>)`](crate::client::fluent_builders::CreatePartition::set_partition_input): <p>A <code>PartitionInput</code> structure defining the partition to be created.</p>
                            /// - On success, responds with [`CreatePartitionOutput`](crate::output::CreatePartitionOutput)
                            /// - On failure, responds with [`SdkError<CreatePartitionError>`](crate::error::CreatePartitionError)
    pub fn create_partition(&self) -> crate::client::fluent_builders::CreatePartition {
                                crate::client::fluent_builders::CreatePartition::new(self.handle.clone())
                            }
}

