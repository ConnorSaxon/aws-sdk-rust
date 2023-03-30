// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`EnableFastSnapshotRestores`](crate::client::fluent_builders::EnableFastSnapshotRestores) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`availability_zones(Vec<String>)`](crate::client::fluent_builders::EnableFastSnapshotRestores::availability_zones) / [`set_availability_zones(Option<Vec<String>>)`](crate::client::fluent_builders::EnableFastSnapshotRestores::set_availability_zones): <p>One or more Availability Zones. For example, <code>us-east-2a</code>.</p>
    ///   - [`source_snapshot_ids(Vec<String>)`](crate::client::fluent_builders::EnableFastSnapshotRestores::source_snapshot_ids) / [`set_source_snapshot_ids(Option<Vec<String>>)`](crate::client::fluent_builders::EnableFastSnapshotRestores::set_source_snapshot_ids): <p>The IDs of one or more snapshots. For example, <code>snap-1234567890abcdef0</code>. You can specify a snapshot that was shared with you from another Amazon Web Services account.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::EnableFastSnapshotRestores::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::EnableFastSnapshotRestores::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`EnableFastSnapshotRestoresOutput`](crate::output::EnableFastSnapshotRestoresOutput) with field(s):
    ///   - [`successful(Option<Vec<EnableFastSnapshotRestoreSuccessItem>>)`](crate::output::EnableFastSnapshotRestoresOutput::successful): <p>Information about the snapshots for which fast snapshot restores were successfully enabled.</p>
    ///   - [`unsuccessful(Option<Vec<EnableFastSnapshotRestoreErrorItem>>)`](crate::output::EnableFastSnapshotRestoresOutput::unsuccessful): <p>Information about the snapshots for which fast snapshot restores could not be enabled.</p>
                            /// - On failure, responds with [`SdkError<EnableFastSnapshotRestoresError>`](crate::error::EnableFastSnapshotRestoresError)
    pub fn enable_fast_snapshot_restores(&self) -> crate::client::fluent_builders::EnableFastSnapshotRestores {
                                crate::client::fluent_builders::EnableFastSnapshotRestores::new(self.handle.clone())
                            }
}

