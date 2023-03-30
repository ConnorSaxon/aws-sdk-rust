// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDashboardForJobRun`](crate::client::fluent_builders::GetDashboardForJobRun) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::GetDashboardForJobRun::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::GetDashboardForJobRun::set_application_id): <p>The ID of the application.</p>
    ///   - [`job_run_id(impl Into<String>)`](crate::client::fluent_builders::GetDashboardForJobRun::job_run_id) / [`set_job_run_id(Option<String>)`](crate::client::fluent_builders::GetDashboardForJobRun::set_job_run_id): <p>The ID of the job run.</p>
                            /// - On success, responds with [`GetDashboardForJobRunOutput`](crate::output::GetDashboardForJobRunOutput) with field(s):
    ///   - [`url(Option<String>)`](crate::output::GetDashboardForJobRunOutput::url): <p>The URL to view job run's dashboard.</p>
                            /// - On failure, responds with [`SdkError<GetDashboardForJobRunError>`](crate::error::GetDashboardForJobRunError)
    pub fn get_dashboard_for_job_run(&self) -> crate::client::fluent_builders::GetDashboardForJobRun {
                                crate::client::fluent_builders::GetDashboardForJobRun::new(self.handle.clone())
                            }
}

