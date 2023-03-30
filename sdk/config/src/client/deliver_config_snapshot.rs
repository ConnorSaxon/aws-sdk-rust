// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeliverConfigSnapshot`](crate::client::fluent_builders::DeliverConfigSnapshot) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`delivery_channel_name(impl Into<String>)`](crate::client::fluent_builders::DeliverConfigSnapshot::delivery_channel_name) / [`set_delivery_channel_name(Option<String>)`](crate::client::fluent_builders::DeliverConfigSnapshot::set_delivery_channel_name): <p>The name of the delivery channel through which the snapshot is delivered.</p>
                            /// - On success, responds with [`DeliverConfigSnapshotOutput`](crate::output::DeliverConfigSnapshotOutput) with field(s):
    ///   - [`config_snapshot_id(Option<String>)`](crate::output::DeliverConfigSnapshotOutput::config_snapshot_id): <p>The ID of the snapshot that is being created.</p>
                            /// - On failure, responds with [`SdkError<DeliverConfigSnapshotError>`](crate::error::DeliverConfigSnapshotError)
    pub fn deliver_config_snapshot(&self) -> crate::client::fluent_builders::DeliverConfigSnapshot {
                                crate::client::fluent_builders::DeliverConfigSnapshot::new(self.handle.clone())
                            }
}

