// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartJobRun`](crate::client::fluent_builders::StartJobRun) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::StartJobRun::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::StartJobRun::set_name): <p>The name of the job run.</p>
    ///   - [`virtual_cluster_id(impl Into<String>)`](crate::client::fluent_builders::StartJobRun::virtual_cluster_id) / [`set_virtual_cluster_id(Option<String>)`](crate::client::fluent_builders::StartJobRun::set_virtual_cluster_id): <p>The virtual cluster ID for which the job run request is submitted.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::StartJobRun::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::StartJobRun::set_client_token): <p>The client idempotency token of the job run request. </p>
    ///   - [`execution_role_arn(impl Into<String>)`](crate::client::fluent_builders::StartJobRun::execution_role_arn) / [`set_execution_role_arn(Option<String>)`](crate::client::fluent_builders::StartJobRun::set_execution_role_arn): <p>The execution role ARN for the job run.</p>
    ///   - [`release_label(impl Into<String>)`](crate::client::fluent_builders::StartJobRun::release_label) / [`set_release_label(Option<String>)`](crate::client::fluent_builders::StartJobRun::set_release_label): <p>The Amazon EMR release version to use for the job run.</p>
    ///   - [`job_driver(JobDriver)`](crate::client::fluent_builders::StartJobRun::job_driver) / [`set_job_driver(Option<JobDriver>)`](crate::client::fluent_builders::StartJobRun::set_job_driver): <p>The job driver for the job run.</p>
    ///   - [`configuration_overrides(ConfigurationOverrides)`](crate::client::fluent_builders::StartJobRun::configuration_overrides) / [`set_configuration_overrides(Option<ConfigurationOverrides>)`](crate::client::fluent_builders::StartJobRun::set_configuration_overrides): <p>The configuration overrides for the job run.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::StartJobRun::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::StartJobRun::set_tags): <p>The tags assigned to job runs.</p>
    ///   - [`job_template_id(impl Into<String>)`](crate::client::fluent_builders::StartJobRun::job_template_id) / [`set_job_template_id(Option<String>)`](crate::client::fluent_builders::StartJobRun::set_job_template_id): <p>The job template ID to be used to start the job run.</p>
    ///   - [`job_template_parameters(HashMap<String, String>)`](crate::client::fluent_builders::StartJobRun::job_template_parameters) / [`set_job_template_parameters(Option<HashMap<String, String>>)`](crate::client::fluent_builders::StartJobRun::set_job_template_parameters): <p>The values of job template parameters to start a job run.</p>
                            /// - On success, responds with [`StartJobRunOutput`](crate::output::StartJobRunOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::StartJobRunOutput::id): <p>This output displays the started job run ID.</p>
    ///   - [`name(Option<String>)`](crate::output::StartJobRunOutput::name): <p>This output displays the name of the started job run.</p>
    ///   - [`arn(Option<String>)`](crate::output::StartJobRunOutput::arn): <p>This output lists the ARN of job run.</p>
    ///   - [`virtual_cluster_id(Option<String>)`](crate::output::StartJobRunOutput::virtual_cluster_id): <p>This output displays the virtual cluster ID for which the job run was submitted.</p>
                            /// - On failure, responds with [`SdkError<StartJobRunError>`](crate::error::StartJobRunError)
    pub fn start_job_run(&self) -> crate::client::fluent_builders::StartJobRun {
                                crate::client::fluent_builders::StartJobRun::new(self.handle.clone())
                            }
}

