// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateReplicationJob`](crate::client::fluent_builders::UpdateReplicationJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`replication_job_id(impl Into<String>)`](crate::client::fluent_builders::UpdateReplicationJob::replication_job_id) / [`set_replication_job_id(Option<String>)`](crate::client::fluent_builders::UpdateReplicationJob::set_replication_job_id): <p>The ID of the replication job.</p>
    ///   - [`frequency(i32)`](crate::client::fluent_builders::UpdateReplicationJob::frequency) / [`set_frequency(Option<i32>)`](crate::client::fluent_builders::UpdateReplicationJob::set_frequency): <p>The time between consecutive replication runs, in hours.</p>
    ///   - [`next_replication_run_start_time(DateTime)`](crate::client::fluent_builders::UpdateReplicationJob::next_replication_run_start_time) / [`set_next_replication_run_start_time(Option<DateTime>)`](crate::client::fluent_builders::UpdateReplicationJob::set_next_replication_run_start_time): <p>The start time of the next replication run.</p>
    ///   - [`license_type(LicenseType)`](crate::client::fluent_builders::UpdateReplicationJob::license_type) / [`set_license_type(Option<LicenseType>)`](crate::client::fluent_builders::UpdateReplicationJob::set_license_type): <p>The license type to be used for the AMI created by a successful replication run.</p>
    ///   - [`role_name(impl Into<String>)`](crate::client::fluent_builders::UpdateReplicationJob::role_name) / [`set_role_name(Option<String>)`](crate::client::fluent_builders::UpdateReplicationJob::set_role_name): <p>The name of the IAM role to be used by Server Migration Service.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateReplicationJob::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateReplicationJob::set_description): <p>The description of the replication job.</p>
    ///   - [`number_of_recent_amis_to_keep(i32)`](crate::client::fluent_builders::UpdateReplicationJob::number_of_recent_amis_to_keep) / [`set_number_of_recent_amis_to_keep(Option<i32>)`](crate::client::fluent_builders::UpdateReplicationJob::set_number_of_recent_amis_to_keep): <p>The maximum number of SMS-created AMIs to retain. The oldest is deleted after the maximum number is reached and a new AMI is created.</p>
    ///   - [`encrypted(bool)`](crate::client::fluent_builders::UpdateReplicationJob::encrypted) / [`set_encrypted(Option<bool>)`](crate::client::fluent_builders::UpdateReplicationJob::set_encrypted): <p>When true, the replication job produces encrypted AMIs. For more information, <code>KmsKeyId</code>.</p>
    ///   - [`kms_key_id(impl Into<String>)`](crate::client::fluent_builders::UpdateReplicationJob::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::client::fluent_builders::UpdateReplicationJob::set_kms_key_id): <p>The ID of the KMS key for replication jobs that produce encrypted AMIs. This value can be any of the following:</p>  <ul>   <li> <p>KMS key ID</p> </li>   <li> <p>KMS key alias</p> </li>   <li> <p>ARN referring to the KMS key ID</p> </li>   <li> <p>ARN referring to the KMS key alias</p> </li>  </ul>  <p>If encrypted is enabled but a KMS key ID is not specified, the customer's default KMS key for Amazon EBS is used.</p>
                            /// - On success, responds with [`UpdateReplicationJobOutput`](crate::output::UpdateReplicationJobOutput)
                            /// - On failure, responds with [`SdkError<UpdateReplicationJobError>`](crate::error::UpdateReplicationJobError)
    pub fn update_replication_job(&self) -> crate::client::fluent_builders::UpdateReplicationJob {
                                crate::client::fluent_builders::UpdateReplicationJob::new(self.handle.clone())
                            }
}

