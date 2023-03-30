// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CompleteSnapshot`](crate::client::fluent_builders::CompleteSnapshot) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`snapshot_id(impl Into<String>)`](crate::client::fluent_builders::CompleteSnapshot::snapshot_id) / [`set_snapshot_id(Option<String>)`](crate::client::fluent_builders::CompleteSnapshot::set_snapshot_id): <p>The ID of the snapshot.</p>
    ///   - [`changed_blocks_count(i32)`](crate::client::fluent_builders::CompleteSnapshot::changed_blocks_count) / [`set_changed_blocks_count(Option<i32>)`](crate::client::fluent_builders::CompleteSnapshot::set_changed_blocks_count): <p>The number of blocks that were written to the snapshot.</p>
    ///   - [`checksum(impl Into<String>)`](crate::client::fluent_builders::CompleteSnapshot::checksum) / [`set_checksum(Option<String>)`](crate::client::fluent_builders::CompleteSnapshot::set_checksum): <p>An aggregated Base-64 SHA256 checksum based on the checksums of each written block.</p>  <p>To generate the aggregated checksum using the linear aggregation method, arrange the checksums for each written block in ascending order of their block index, concatenate them to form a single string, and then generate the checksum on the entire string using the SHA256 algorithm.</p>
    ///   - [`checksum_algorithm(ChecksumAlgorithm)`](crate::client::fluent_builders::CompleteSnapshot::checksum_algorithm) / [`set_checksum_algorithm(Option<ChecksumAlgorithm>)`](crate::client::fluent_builders::CompleteSnapshot::set_checksum_algorithm): <p>The algorithm used to generate the checksum. Currently, the only supported algorithm is <code>SHA256</code>.</p>
    ///   - [`checksum_aggregation_method(ChecksumAggregationMethod)`](crate::client::fluent_builders::CompleteSnapshot::checksum_aggregation_method) / [`set_checksum_aggregation_method(Option<ChecksumAggregationMethod>)`](crate::client::fluent_builders::CompleteSnapshot::set_checksum_aggregation_method): <p>The aggregation method used to generate the checksum. Currently, the only supported aggregation method is <code>LINEAR</code>.</p>
                            /// - On success, responds with [`CompleteSnapshotOutput`](crate::output::CompleteSnapshotOutput) with field(s):
    ///   - [`status(Option<Status>)`](crate::output::CompleteSnapshotOutput::status): <p>The status of the snapshot.</p>
                            /// - On failure, responds with [`SdkError<CompleteSnapshotError>`](crate::error::CompleteSnapshotError)
    pub fn complete_snapshot(&self) -> crate::client::fluent_builders::CompleteSnapshot {
                                crate::client::fluent_builders::CompleteSnapshot::new(self.handle.clone())
                            }
}

