// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifySnapshotTier`](crate::client::fluent_builders::ModifySnapshotTier) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`snapshot_id(impl Into<String>)`](crate::client::fluent_builders::ModifySnapshotTier::snapshot_id) / [`set_snapshot_id(Option<String>)`](crate::client::fluent_builders::ModifySnapshotTier::set_snapshot_id): <p>The ID of the snapshot.</p>
    ///   - [`storage_tier(TargetStorageTier)`](crate::client::fluent_builders::ModifySnapshotTier::storage_tier) / [`set_storage_tier(Option<TargetStorageTier>)`](crate::client::fluent_builders::ModifySnapshotTier::set_storage_tier): <p>The name of the storage tier. You must specify <code>archive</code>.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::ModifySnapshotTier::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::ModifySnapshotTier::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`ModifySnapshotTierOutput`](crate::output::ModifySnapshotTierOutput) with field(s):
    ///   - [`snapshot_id(Option<String>)`](crate::output::ModifySnapshotTierOutput::snapshot_id): <p>The ID of the snapshot.</p>
    ///   - [`tiering_start_time(Option<DateTime>)`](crate::output::ModifySnapshotTierOutput::tiering_start_time): <p>The date and time when the archive process was started.</p>
                            /// - On failure, responds with [`SdkError<ModifySnapshotTierError>`](crate::error::ModifySnapshotTierError)
    pub fn modify_snapshot_tier(&self) -> crate::client::fluent_builders::ModifySnapshotTier {
                                crate::client::fluent_builders::ModifySnapshotTier::new(self.handle.clone())
                            }
}

