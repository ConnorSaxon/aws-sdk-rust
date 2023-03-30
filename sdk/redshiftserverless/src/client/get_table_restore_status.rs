// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetTableRestoreStatus`](crate::client::fluent_builders::GetTableRestoreStatus) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`table_restore_request_id(impl Into<String>)`](crate::client::fluent_builders::GetTableRestoreStatus::table_restore_request_id) / [`set_table_restore_request_id(Option<String>)`](crate::client::fluent_builders::GetTableRestoreStatus::set_table_restore_request_id): <p>The ID of the <code>RestoreTableFromSnapshot</code> request to return status for.</p>
                            /// - On success, responds with [`GetTableRestoreStatusOutput`](crate::output::GetTableRestoreStatusOutput) with field(s):
    ///   - [`table_restore_status(Option<TableRestoreStatus>)`](crate::output::GetTableRestoreStatusOutput::table_restore_status): <p>The returned <code>TableRestoreStatus</code> object that contains information about the status of your <code>RestoreTableFromSnapshot</code> request.</p>
                            /// - On failure, responds with [`SdkError<GetTableRestoreStatusError>`](crate::error::GetTableRestoreStatusError)
    pub fn get_table_restore_status(&self) -> crate::client::fluent_builders::GetTableRestoreStatus {
                                crate::client::fluent_builders::GetTableRestoreStatus::new(self.handle.clone())
                            }
}

