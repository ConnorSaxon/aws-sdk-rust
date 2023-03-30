// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteApplicationSnapshot`](crate::client::fluent_builders::DeleteApplicationSnapshot) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::client::fluent_builders::DeleteApplicationSnapshot::application_name) / [`set_application_name(Option<String>)`](crate::client::fluent_builders::DeleteApplicationSnapshot::set_application_name): <p>The name of an existing application.</p>
    ///   - [`snapshot_name(impl Into<String>)`](crate::client::fluent_builders::DeleteApplicationSnapshot::snapshot_name) / [`set_snapshot_name(Option<String>)`](crate::client::fluent_builders::DeleteApplicationSnapshot::set_snapshot_name): <p>The identifier for the snapshot delete.</p>
    ///   - [`snapshot_creation_timestamp(DateTime)`](crate::client::fluent_builders::DeleteApplicationSnapshot::snapshot_creation_timestamp) / [`set_snapshot_creation_timestamp(Option<DateTime>)`](crate::client::fluent_builders::DeleteApplicationSnapshot::set_snapshot_creation_timestamp): <p>The creation timestamp of the application snapshot to delete. You can retrieve this value using or .</p>
                            /// - On success, responds with [`DeleteApplicationSnapshotOutput`](crate::output::DeleteApplicationSnapshotOutput)
                            /// - On failure, responds with [`SdkError<DeleteApplicationSnapshotError>`](crate::error::DeleteApplicationSnapshotError)
    pub fn delete_application_snapshot(&self) -> crate::client::fluent_builders::DeleteApplicationSnapshot {
                                crate::client::fluent_builders::DeleteApplicationSnapshot::new(self.handle.clone())
                            }
}

