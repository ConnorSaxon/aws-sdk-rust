// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CopyBackup`](crate::client::fluent_builders::CopyBackup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CopyBackup::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CopyBackup::set_client_request_token): <p>(Optional) An idempotency token for resource creation, in a string of up to 64 ASCII characters. This token is automatically filled on your behalf when you use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p>
    ///   - [`source_backup_id(impl Into<String>)`](crate::client::fluent_builders::CopyBackup::source_backup_id) / [`set_source_backup_id(Option<String>)`](crate::client::fluent_builders::CopyBackup::set_source_backup_id): <p>The ID of the source backup. Specifies the ID of the backup that's being copied.</p>
    ///   - [`source_region(impl Into<String>)`](crate::client::fluent_builders::CopyBackup::source_region) / [`set_source_region(Option<String>)`](crate::client::fluent_builders::CopyBackup::set_source_region): <p>The source Amazon Web Services Region of the backup. Specifies the Amazon Web Services Region from which the backup is being copied. The source and destination Regions must be in the same Amazon Web Services partition. If you don't specify a Region, <code>SourceRegion</code> defaults to the Region where the request is sent from (in-Region copy).</p>
    ///   - [`kms_key_id(impl Into<String>)`](crate::client::fluent_builders::CopyBackup::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::client::fluent_builders::CopyBackup::set_kms_key_id): <p>Specifies the ID of the Key Management Service (KMS) key to use for encrypting data on Amazon FSx file systems, as follows:</p>  <ul>   <li> <p>Amazon FSx for Lustre <code>PERSISTENT_1</code> and <code>PERSISTENT_2</code> deployment types only.</p> <p> <code>SCRATCH_1</code> and <code>SCRATCH_2</code> types are encrypted using the Amazon FSx service KMS key for your account.</p> </li>   <li> <p>Amazon FSx for NetApp ONTAP</p> </li>   <li> <p>Amazon FSx for OpenZFS</p> </li>   <li> <p>Amazon FSx for Windows File Server</p> </li>  </ul>  <p>If a <code>KmsKeyId</code> isn't specified, the Amazon FSx-managed KMS key for your account is used. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_Encrypt.html">Encrypt</a> in the <i>Key Management Service API Reference</i>.</p>
    ///   - [`copy_tags(bool)`](crate::client::fluent_builders::CopyBackup::copy_tags) / [`set_copy_tags(Option<bool>)`](crate::client::fluent_builders::CopyBackup::set_copy_tags): <p>A Boolean flag indicating whether tags from the source backup should be copied to the backup copy. This value defaults to <code>false</code>.</p>  <p>If you set <code>CopyTags</code> to <code>true</code> and the source backup has existing tags, you can use the <code>Tags</code> parameter to create new tags, provided that the sum of the source backup tags and the new tags doesn't exceed 50. Both sets of tags are merged. If there are tag conflicts (for example, two tags with the same key but different values), the tags created with the <code>Tags</code> parameter take precedence.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CopyBackup::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CopyBackup::set_tags): <p>A list of <code>Tag</code> values, with a maximum of 50 elements.</p>
                            /// - On success, responds with [`CopyBackupOutput`](crate::output::CopyBackupOutput) with field(s):
    ///   - [`backup(Option<Backup>)`](crate::output::CopyBackupOutput::backup): <p>A backup of an Amazon FSx for Windows File Server, Amazon FSx for Lustre file system, Amazon FSx for NetApp ONTAP volume, or Amazon FSx for OpenZFS file system.</p>
                            /// - On failure, responds with [`SdkError<CopyBackupError>`](crate::error::CopyBackupError)
    pub fn copy_backup(&self) -> crate::client::fluent_builders::CopyBackup {
                                crate::client::fluent_builders::CopyBackup::new(self.handle.clone())
                            }
}

