// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CopyBackupToRegion`](crate::client::fluent_builders::CopyBackupToRegion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`destination_region(impl Into<String>)`](crate::client::fluent_builders::CopyBackupToRegion::destination_region) / [`set_destination_region(Option<String>)`](crate::client::fluent_builders::CopyBackupToRegion::set_destination_region): <p>The AWS region that will contain your copied CloudHSM cluster backup.</p>
    ///   - [`backup_id(impl Into<String>)`](crate::client::fluent_builders::CopyBackupToRegion::backup_id) / [`set_backup_id(Option<String>)`](crate::client::fluent_builders::CopyBackupToRegion::set_backup_id): <p>The ID of the backup that will be copied to the destination region. </p>
    ///   - [`tag_list(Vec<Tag>)`](crate::client::fluent_builders::CopyBackupToRegion::tag_list) / [`set_tag_list(Option<Vec<Tag>>)`](crate::client::fluent_builders::CopyBackupToRegion::set_tag_list): <p>Tags to apply to the destination backup during creation. If you specify tags, only these tags will be applied to the destination backup. If you do not specify tags, the service copies tags from the source backup to the destination backup.</p>
                            /// - On success, responds with [`CopyBackupToRegionOutput`](crate::output::CopyBackupToRegionOutput) with field(s):
    ///   - [`destination_backup(Option<DestinationBackup>)`](crate::output::CopyBackupToRegionOutput::destination_backup): <p>Information on the backup that will be copied to the destination region, including CreateTimestamp, SourceBackup, SourceCluster, and Source Region. CreateTimestamp of the destination backup will be the same as that of the source backup.</p>  <p>You will need to use the <code>sourceBackupID</code> returned in this operation to use the <code>DescribeBackups</code> operation on the backup that will be copied to the destination region.</p>
                            /// - On failure, responds with [`SdkError<CopyBackupToRegionError>`](crate::error::CopyBackupToRegionError)
    pub fn copy_backup_to_region(&self) -> crate::client::fluent_builders::CopyBackupToRegion {
                                crate::client::fluent_builders::CopyBackupToRegion::new(self.handle.clone())
                            }
}

