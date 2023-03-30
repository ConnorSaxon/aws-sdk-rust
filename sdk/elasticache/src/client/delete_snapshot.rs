// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteSnapshot`](crate::client::fluent_builders::DeleteSnapshot) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`snapshot_name(impl Into<String>)`](crate::client::fluent_builders::DeleteSnapshot::snapshot_name) / [`set_snapshot_name(Option<String>)`](crate::client::fluent_builders::DeleteSnapshot::set_snapshot_name): <p>The name of the snapshot to be deleted.</p>
                            /// - On success, responds with [`DeleteSnapshotOutput`](crate::output::DeleteSnapshotOutput) with field(s):
    ///   - [`snapshot(Option<Snapshot>)`](crate::output::DeleteSnapshotOutput::snapshot): <p>Represents a copy of an entire Redis cluster as of the time when the snapshot was taken.</p>
                            /// - On failure, responds with [`SdkError<DeleteSnapshotError>`](crate::error::DeleteSnapshotError)
    pub fn delete_snapshot(&self) -> crate::client::fluent_builders::DeleteSnapshot {
                                crate::client::fluent_builders::DeleteSnapshot::new(self.handle.clone())
                            }
}

