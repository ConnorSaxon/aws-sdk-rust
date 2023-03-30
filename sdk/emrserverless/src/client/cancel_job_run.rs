// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelJobRun`](crate::client::fluent_builders::CancelJobRun) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::CancelJobRun::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::CancelJobRun::set_application_id): <p>The ID of the application on which the job run will be canceled.</p>
    ///   - [`job_run_id(impl Into<String>)`](crate::client::fluent_builders::CancelJobRun::job_run_id) / [`set_job_run_id(Option<String>)`](crate::client::fluent_builders::CancelJobRun::set_job_run_id): <p>The ID of the job run to cancel.</p>
                            /// - On success, responds with [`CancelJobRunOutput`](crate::output::CancelJobRunOutput) with field(s):
    ///   - [`application_id(Option<String>)`](crate::output::CancelJobRunOutput::application_id): <p>The output contains the application ID on which the job run is cancelled.</p>
    ///   - [`job_run_id(Option<String>)`](crate::output::CancelJobRunOutput::job_run_id): <p>The output contains the ID of the cancelled job run.</p>
                            /// - On failure, responds with [`SdkError<CancelJobRunError>`](crate::error::CancelJobRunError)
    pub fn cancel_job_run(&self) -> crate::client::fluent_builders::CancelJobRun {
                                crate::client::fluent_builders::CancelJobRun::new(self.handle.clone())
                            }
}

