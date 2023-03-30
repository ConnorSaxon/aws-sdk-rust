// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateRecoveryPointLifecycle`](crate::client::fluent_builders::UpdateRecoveryPointLifecycle) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`backup_vault_name(impl Into<String>)`](crate::client::fluent_builders::UpdateRecoveryPointLifecycle::backup_vault_name) / [`set_backup_vault_name(Option<String>)`](crate::client::fluent_builders::UpdateRecoveryPointLifecycle::set_backup_vault_name): <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    ///   - [`recovery_point_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateRecoveryPointLifecycle::recovery_point_arn) / [`set_recovery_point_arn(Option<String>)`](crate::client::fluent_builders::UpdateRecoveryPointLifecycle::set_recovery_point_arn): <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    ///   - [`lifecycle(Lifecycle)`](crate::client::fluent_builders::UpdateRecoveryPointLifecycle::lifecycle) / [`set_lifecycle(Option<Lifecycle>)`](crate::client::fluent_builders::UpdateRecoveryPointLifecycle::set_lifecycle): <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. Backup transitions and expires backups automatically according to the lifecycle that you define. </p>  <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “retention” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold. </p>
                            /// - On success, responds with [`UpdateRecoveryPointLifecycleOutput`](crate::output::UpdateRecoveryPointLifecycleOutput) with field(s):
    ///   - [`backup_vault_arn(Option<String>)`](crate::output::UpdateRecoveryPointLifecycleOutput::backup_vault_arn): <p>An ARN that uniquely identifies a backup vault; for example, <code>arn:aws:backup:us-east-1:123456789012:vault:aBackupVault</code>.</p>
    ///   - [`recovery_point_arn(Option<String>)`](crate::output::UpdateRecoveryPointLifecycleOutput::recovery_point_arn): <p>An Amazon Resource Name (ARN) that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    ///   - [`lifecycle(Option<Lifecycle>)`](crate::output::UpdateRecoveryPointLifecycleOutput::lifecycle): <p>The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. Backup transitions and expires backups automatically according to the lifecycle that you define.</p>  <p>Backups transitioned to cold storage must be stored in cold storage for a minimum of 90 days. Therefore, the “retention” setting must be 90 days greater than the “transition to cold after days” setting. The “transition to cold after days” setting cannot be changed after a backup has been transitioned to cold.</p>  <p>Resource types that are able to be transitioned to cold storage are listed in the "Lifecycle to cold storage" section of the <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/whatisbackup.html#features-by-resource"> Feature availability by resource</a> table. Backup ignores this expression for other resource types.</p>
    ///   - [`calculated_lifecycle(Option<CalculatedLifecycle>)`](crate::output::UpdateRecoveryPointLifecycleOutput::calculated_lifecycle): <p>A <code>CalculatedLifecycle</code> object containing <code>DeleteAt</code> and <code>MoveToColdStorageAt</code> timestamps.</p>
                            /// - On failure, responds with [`SdkError<UpdateRecoveryPointLifecycleError>`](crate::error::UpdateRecoveryPointLifecycleError)
    pub fn update_recovery_point_lifecycle(&self) -> crate::client::fluent_builders::UpdateRecoveryPointLifecycle {
                                crate::client::fluent_builders::UpdateRecoveryPointLifecycle::new(self.handle.clone())
                            }
}

