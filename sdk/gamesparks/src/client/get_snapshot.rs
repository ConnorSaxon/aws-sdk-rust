// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetSnapshot`](crate::client::fluent_builders::GetSnapshot) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`game_name(impl Into<String>)`](crate::client::fluent_builders::GetSnapshot::game_name) / [`set_game_name(Option<String>)`](crate::client::fluent_builders::GetSnapshot::set_game_name): <p>The name of the game.</p>
    ///   - [`snapshot_id(impl Into<String>)`](crate::client::fluent_builders::GetSnapshot::snapshot_id) / [`set_snapshot_id(Option<String>)`](crate::client::fluent_builders::GetSnapshot::set_snapshot_id): <p>The identifier of the snapshot.</p>
    ///   - [`sections(Vec<String>)`](crate::client::fluent_builders::GetSnapshot::sections) / [`set_sections(Option<Vec<String>>)`](crate::client::fluent_builders::GetSnapshot::set_sections): <p>The list of game configuration sections to be described.</p>
                            /// - On success, responds with [`GetSnapshotOutput`](crate::output::GetSnapshotOutput) with field(s):
    ///   - [`snapshot(Option<SnapshotDetails>)`](crate::output::GetSnapshotOutput::snapshot): <p>Properties that provide details of the snapshot.</p>
                            /// - On failure, responds with [`SdkError<GetSnapshotError>`](crate::error::GetSnapshotError)
    pub fn get_snapshot(&self) -> crate::client::fluent_builders::GetSnapshot {
                                crate::client::fluent_builders::GetSnapshot::new(self.handle.clone())
                            }
}

