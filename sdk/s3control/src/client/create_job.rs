// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateJob`](crate::client::fluent_builders::CreateJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::CreateJob::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::CreateJob::set_account_id): <p>The Amazon Web Services account ID that creates the job.</p>
    ///   - [`confirmation_required(bool)`](crate::client::fluent_builders::CreateJob::confirmation_required) / [`set_confirmation_required(Option<bool>)`](crate::client::fluent_builders::CreateJob::set_confirmation_required): <p>Indicates whether confirmation is required before Amazon S3 runs the job. Confirmation is only required for jobs created through the Amazon S3 console.</p>
    ///   - [`operation(JobOperation)`](crate::client::fluent_builders::CreateJob::operation) / [`set_operation(Option<JobOperation>)`](crate::client::fluent_builders::CreateJob::set_operation): <p>The action that you want this job to perform on every object listed in the manifest. For more information about the available actions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/batch-ops-actions.html">Operations</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`report(JobReport)`](crate::client::fluent_builders::CreateJob::report) / [`set_report(Option<JobReport>)`](crate::client::fluent_builders::CreateJob::set_report): <p>Configuration parameters for the optional job-completion report.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CreateJob::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CreateJob::set_client_request_token): <p>An idempotency token to ensure that you don't accidentally submit the same request twice. You can use any string up to the maximum length.</p>
    ///   - [`manifest(JobManifest)`](crate::client::fluent_builders::CreateJob::manifest) / [`set_manifest(Option<JobManifest>)`](crate::client::fluent_builders::CreateJob::set_manifest): <p>Configuration parameters for the manifest.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateJob::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateJob::set_description): <p>A description for this job. You can use any string within the permitted length. Descriptions don't need to be unique and can be used for multiple jobs.</p>
    ///   - [`priority(i32)`](crate::client::fluent_builders::CreateJob::priority) / [`set_priority(Option<i32>)`](crate::client::fluent_builders::CreateJob::set_priority): <p>The numerical priority for this job. Higher numbers indicate higher priority.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateJob::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::CreateJob::set_role_arn): <p>The Amazon Resource Name (ARN) for the Identity and Access Management (IAM) role that Batch Operations will use to run this job's action on every object in the manifest.</p>
    ///   - [`tags(Vec<S3Tag>)`](crate::client::fluent_builders::CreateJob::tags) / [`set_tags(Option<Vec<S3Tag>>)`](crate::client::fluent_builders::CreateJob::set_tags): <p>A set of tags to associate with the S3 Batch Operations job. This is an optional parameter. </p>
    ///   - [`manifest_generator(JobManifestGenerator)`](crate::client::fluent_builders::CreateJob::manifest_generator) / [`set_manifest_generator(Option<JobManifestGenerator>)`](crate::client::fluent_builders::CreateJob::set_manifest_generator): <p>The attribute container for the ManifestGenerator details. Jobs must be created with either a manifest file or a ManifestGenerator, but not both.</p>
                            /// - On success, responds with [`CreateJobOutput`](crate::output::CreateJobOutput) with field(s):
    ///   - [`job_id(Option<String>)`](crate::output::CreateJobOutput::job_id): <p>The ID for this job. Amazon S3 generates this ID automatically and returns it after a successful <code>Create Job</code> request.</p>
                            /// - On failure, responds with [`SdkError<CreateJobError>`](crate::error::CreateJobError)
    pub fn create_job(&self) -> crate::client::fluent_builders::CreateJob {
                                crate::client::fluent_builders::CreateJob::new(self.handle.clone())
                            }
}

