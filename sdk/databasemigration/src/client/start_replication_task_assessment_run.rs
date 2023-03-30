// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartReplicationTaskAssessmentRun`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`replication_task_arn(impl Into<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::replication_task_arn) / [`set_replication_task_arn(Option<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::set_replication_task_arn): <p>Amazon Resource Name (ARN) of the migration task associated with the premigration assessment run that you want to start.</p>
    ///   - [`service_access_role_arn(impl Into<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::service_access_role_arn) / [`set_service_access_role_arn(Option<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::set_service_access_role_arn): <p>ARN of the service role needed to start the assessment run. The role must allow the <code>iam:PassRole</code> action.</p>
    ///   - [`result_location_bucket(impl Into<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::result_location_bucket) / [`set_result_location_bucket(Option<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::set_result_location_bucket): <p>Amazon S3 bucket where you want DMS to store the results of this assessment run.</p>
    ///   - [`result_location_folder(impl Into<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::result_location_folder) / [`set_result_location_folder(Option<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::set_result_location_folder): <p>Folder within an Amazon S3 bucket where you want DMS to store the results of this assessment run.</p>
    ///   - [`result_encryption_mode(impl Into<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::result_encryption_mode) / [`set_result_encryption_mode(Option<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::set_result_encryption_mode): <p>Encryption mode that you can specify to encrypt the results of this assessment run. If you don't specify this request parameter, DMS stores the assessment run results without encryption. You can specify one of the options following:</p>  <ul>   <li> <p> <code>"SSE_S3"</code> – The server-side encryption provided as a default by Amazon S3.</p> </li>   <li> <p> <code>"SSE_KMS"</code> – Key Management Service (KMS) encryption. This encryption can use either a custom KMS encryption key that you specify or the default KMS encryption key that DMS provides.</p> </li>  </ul>
    ///   - [`result_kms_key_arn(impl Into<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::result_kms_key_arn) / [`set_result_kms_key_arn(Option<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::set_result_kms_key_arn): <p>ARN of a custom KMS encryption key that you specify when you set <code>ResultEncryptionMode</code> to <code>"SSE_KMS</code>".</p>
    ///   - [`assessment_run_name(impl Into<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::assessment_run_name) / [`set_assessment_run_name(Option<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::set_assessment_run_name): <p>Unique name to identify the assessment run.</p>
    ///   - [`include_only(Vec<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::include_only) / [`set_include_only(Option<Vec<String>>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::set_include_only): <p>Space-separated list of names for specific individual assessments that you want to include. These names come from the default list of individual assessments that DMS supports for the associated migration task. This task is specified by <code>ReplicationTaskArn</code>.</p> <note>   <p>You can't set a value for <code>IncludeOnly</code> if you also set a value for <code>Exclude</code> in the API operation. </p>   <p>To identify the names of the default individual assessments that DMS supports for the associated migration task, run the <code>DescribeApplicableIndividualAssessments</code> operation using its own <code>ReplicationTaskArn</code> request parameter.</p>  </note>
    ///   - [`exclude(Vec<String>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::exclude) / [`set_exclude(Option<Vec<String>>)`](crate::client::fluent_builders::StartReplicationTaskAssessmentRun::set_exclude): <p>Space-separated list of names for specific individual assessments that you want to exclude. These names come from the default list of individual assessments that DMS supports for the associated migration task. This task is specified by <code>ReplicationTaskArn</code>.</p> <note>   <p>You can't set a value for <code>Exclude</code> if you also set a value for <code>IncludeOnly</code> in the API operation.</p>   <p>To identify the names of the default individual assessments that DMS supports for the associated migration task, run the <code>DescribeApplicableIndividualAssessments</code> operation using its own <code>ReplicationTaskArn</code> request parameter.</p>  </note>
                            /// - On success, responds with [`StartReplicationTaskAssessmentRunOutput`](crate::output::StartReplicationTaskAssessmentRunOutput) with field(s):
    ///   - [`replication_task_assessment_run(Option<ReplicationTaskAssessmentRun>)`](crate::output::StartReplicationTaskAssessmentRunOutput::replication_task_assessment_run): <p>The premigration assessment run that was started.</p>
                            /// - On failure, responds with [`SdkError<StartReplicationTaskAssessmentRunError>`](crate::error::StartReplicationTaskAssessmentRunError)
    pub fn start_replication_task_assessment_run(&self) -> crate::client::fluent_builders::StartReplicationTaskAssessmentRun {
                                crate::client::fluent_builders::StartReplicationTaskAssessmentRun::new(self.handle.clone())
                            }
}

