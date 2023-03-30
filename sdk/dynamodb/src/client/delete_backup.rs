// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteBackup`](crate::client::fluent_builders::DeleteBackup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`backup_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteBackup::backup_arn) / [`set_backup_arn(Option<String>)`](crate::client::fluent_builders::DeleteBackup::set_backup_arn): <p>The ARN associated with the backup.</p>
                            /// - On success, responds with [`DeleteBackupOutput`](crate::output::DeleteBackupOutput) with field(s):
    ///   - [`backup_description(Option<BackupDescription>)`](crate::output::DeleteBackupOutput::backup_description): <p>Contains the description of the backup created for the table.</p>
                            /// - On failure, responds with [`SdkError<DeleteBackupError>`](crate::error::DeleteBackupError)
    pub fn delete_backup(&self) -> crate::client::fluent_builders::DeleteBackup {
                                crate::client::fluent_builders::DeleteBackup::new(self.handle.clone())
                            }
}

