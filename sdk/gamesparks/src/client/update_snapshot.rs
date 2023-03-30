// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateSnapshot`](crate::client::fluent_builders::UpdateSnapshot) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`game_name(impl Into<String>)`](crate::client::fluent_builders::UpdateSnapshot::game_name) / [`set_game_name(Option<String>)`](crate::client::fluent_builders::UpdateSnapshot::set_game_name): <p>The name of the game.</p>
    ///   - [`snapshot_id(impl Into<String>)`](crate::client::fluent_builders::UpdateSnapshot::snapshot_id) / [`set_snapshot_id(Option<String>)`](crate::client::fluent_builders::UpdateSnapshot::set_snapshot_id): <p>The identifier of the snapshot.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateSnapshot::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateSnapshot::set_description): <p>The description of the snapshot.</p>
                            /// - On success, responds with [`UpdateSnapshotOutput`](crate::output::UpdateSnapshotOutput) with field(s):
    ///   - [`snapshot(Option<SnapshotDetails>)`](crate::output::UpdateSnapshotOutput::snapshot): <p>Properties that provide details of the updated snapshot.</p>
                            /// - On failure, responds with [`SdkError<UpdateSnapshotError>`](crate::error::UpdateSnapshotError)
    pub fn update_snapshot(&self) -> crate::client::fluent_builders::UpdateSnapshot {
                                crate::client::fluent_builders::UpdateSnapshot::new(self.handle.clone())
                            }
}

