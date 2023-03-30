// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateReplicationJob`](crate::client::fluent_builders::CreateReplicationJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`server_id(impl Into<String>)`](crate::client::fluent_builders::CreateReplicationJob::server_id) / [`set_server_id(Option<String>)`](crate::client::fluent_builders::CreateReplicationJob::set_server_id): <p>The ID of the server.</p>
    ///   - [`seed_replication_time(DateTime)`](crate::client::fluent_builders::CreateReplicationJob::seed_replication_time) / [`set_seed_replication_time(Option<DateTime>)`](crate::client::fluent_builders::CreateReplicationJob::set_seed_replication_time): <p>The seed replication time.</p>
    ///   - [`frequency(i32)`](crate::client::fluent_builders::CreateReplicationJob::frequency) / [`set_frequency(Option<i32>)`](crate::client::fluent_builders::CreateReplicationJob::set_frequency): <p>The time between consecutive replication runs, in hours.</p>
    ///   - [`run_once(bool)`](crate::client::fluent_builders::CreateReplicationJob::run_once) / [`set_run_once(Option<bool>)`](crate::client::fluent_builders::CreateReplicationJob::set_run_once): <p>Indicates whether to run the replication job one time.</p>
    ///   - [`license_type(LicenseType)`](crate::client::fluent_builders::CreateReplicationJob::license_type) / [`set_license_type(Option<LicenseType>)`](crate::client::fluent_builders::CreateReplicationJob::set_license_type): <p>The license type to be used for the AMI created by a successful replication run.</p>
    ///   - [`role_name(impl Into<String>)`](crate::client::fluent_builders::CreateReplicationJob::role_name) / [`set_role_name(Option<String>)`](crate::client::fluent_builders::CreateReplicationJob::set_role_name): <p>The name of the IAM role to be used by the Server Migration Service.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateReplicationJob::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateReplicationJob::set_description): <p>The description of the replication job.</p>
    ///   - [`number_of_recent_amis_to_keep(i32)`](crate::client::fluent_builders::CreateReplicationJob::number_of_recent_amis_to_keep) / [`set_number_of_recent_amis_to_keep(Option<i32>)`](crate::client::fluent_builders::CreateReplicationJob::set_number_of_recent_amis_to_keep): <p>The maximum number of SMS-created AMIs to retain. The oldest is deleted after the maximum number is reached and a new AMI is created.</p>
    ///   - [`encrypted(bool)`](crate::client::fluent_builders::CreateReplicationJob::encrypted) / [`set_encrypted(Option<bool>)`](crate::client::fluent_builders::CreateReplicationJob::set_encrypted): <p>Indicates whether the replication job produces encrypted AMIs.</p>
    ///   - [`kms_key_id(impl Into<String>)`](crate::client::fluent_builders::CreateReplicationJob::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::client::fluent_builders::CreateReplicationJob::set_kms_key_id): <p>The ID of the KMS key for replication jobs that produce encrypted AMIs. This value can be any of the following:</p>  <ul>   <li> <p>KMS key ID</p> </li>   <li> <p>KMS key alias</p> </li>   <li> <p>ARN referring to the KMS key ID</p> </li>   <li> <p>ARN referring to the KMS key alias</p> </li>  </ul>  <p> If encrypted is <i>true</i> but a KMS key ID is not specified, the customer's default KMS key for Amazon EBS is used. </p>
                            /// - On success, responds with [`CreateReplicationJobOutput`](crate::output::CreateReplicationJobOutput) with field(s):
    ///   - [`replication_job_id(Option<String>)`](crate::output::CreateReplicationJobOutput::replication_job_id): <p>The unique identifier of the replication job.</p>
                            /// - On failure, responds with [`SdkError<CreateReplicationJobError>`](crate::error::CreateReplicationJobError)
    pub fn create_replication_job(&self) -> crate::client::fluent_builders::CreateReplicationJob {
                                crate::client::fluent_builders::CreateReplicationJob::new(self.handle.clone())
                            }
}

