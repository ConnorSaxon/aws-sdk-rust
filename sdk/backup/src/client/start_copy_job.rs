// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartCopyJob`](crate::client::fluent_builders::StartCopyJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`recovery_point_arn(impl Into<String>)`](crate::client::fluent_builders::StartCopyJob::recovery_point_arn) / [`set_recovery_point_arn(Option<String>)`](crate::client::fluent_builders::StartCopyJob::set_recovery_point_arn): <p>An ARN that uniquely identifies a recovery point to use for the copy job; for example, arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45. </p>
    ///   - [`source_backup_vault_name(impl Into<String>)`](crate::client::fluent_builders::StartCopyJob::source_backup_vault_name) / [`set_source_backup_vault_name(Option<String>)`](crate::client::fluent_builders::StartCopyJob::set_source_backup_vault_name): <p>The name of a logical source container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    ///   - [`destination_backup_vault_arn(impl Into<String>)`](crate::client::fluent_builders::StartCopyJob::destination_backup_vault_arn) / [`set_destination_backup_vault_arn(Option<String>)`](crate::client::fluent_builders::StartCopyJob::set_destination_backup_vault_arn): <p>An Amazon Resource Name (ARN) that uniquely identifies a destination backup vault to copy to; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    ///   - [`iam_role_arn(impl Into<String>)`](crate::client::fluent_builders::StartCopyJob::iam_role_arn) / [`set_iam_role_arn(Option<String>)`](crate::client::fluent_builders::StartCopyJob::set_iam_role_arn): <p>Specifies the IAM role ARN used to copy the target recovery point; for example, <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    ///   - [`idempotency_token(impl Into<String>)`](crate::client::fluent_builders::StartCopyJob::idempotency_token) / [`set_idempotency_token(Option<String>)`](crate::client::fluent_builders::StartCopyJob::set_idempotency_token): <p>A customer-chosen string that you can use to distinguish between otherwise identical calls to <code>StartCopyJob</code>. Retrying a successful request with the same idempotency token results in a success message with no action taken.</p>
    ///   - [`lifecycle(Lifecycle)`](crate::client::fluent_builders::StartCopyJob::lifecycle) / [`set_lifecycle(Option<Lifecycle>)`](crate::client::fluent_builders::StartCopyJob::set_lifecycle): <p>Contains an array of <code>Transition</code> objects specifying how long in days before a recovery point transitions to cold storage or is deleted.</p>  <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, on the console, the “retention” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold.</p>  <p>Resource types that are able to be transitioned to cold storage are listed in the "Lifecycle to cold storage" section of the <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/whatisbackup.html#features-by-resource"> Feature availability by resource</a> table. Backup ignores this expression for other resource types.</p>
                            /// - On success, responds with [`StartCopyJobOutput`](crate::output::StartCopyJobOutput) with field(s):
    ///   - [`copy_job_id(Option<String>)`](crate::output::StartCopyJobOutput::copy_job_id): <p>Uniquely identifies a copy job.</p>
    ///   - [`creation_date(Option<DateTime>)`](crate::output::StartCopyJobOutput::creation_date): <p>The date and time that a copy job is created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreationDate</code> is accurate to milliseconds. For example, the value 1516925490.087 represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    ///   - [`is_parent(bool)`](crate::output::StartCopyJobOutput::is_parent): <p>This is a returned boolean value indicating this is a parent (composite) copy job.</p>
                            /// - On failure, responds with [`SdkError<StartCopyJobError>`](crate::error::StartCopyJobError)
    pub fn start_copy_job(&self) -> crate::client::fluent_builders::StartCopyJob {
                                crate::client::fluent_builders::StartCopyJob::new(self.handle.clone())
                            }
}

