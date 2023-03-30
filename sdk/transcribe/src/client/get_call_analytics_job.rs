// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetCallAnalyticsJob`](crate::client::fluent_builders::GetCallAnalyticsJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`call_analytics_job_name(impl Into<String>)`](crate::client::fluent_builders::GetCallAnalyticsJob::call_analytics_job_name) / [`set_call_analytics_job_name(Option<String>)`](crate::client::fluent_builders::GetCallAnalyticsJob::set_call_analytics_job_name): <p>The name of the Call Analytics job you want information about. Job names are case sensitive.</p>
                            /// - On success, responds with [`GetCallAnalyticsJobOutput`](crate::output::GetCallAnalyticsJobOutput) with field(s):
    ///   - [`call_analytics_job(Option<CallAnalyticsJob>)`](crate::output::GetCallAnalyticsJobOutput::call_analytics_job): <p>Provides detailed information about the specified Call Analytics job, including job status and, if applicable, failure reason.</p>
                            /// - On failure, responds with [`SdkError<GetCallAnalyticsJobError>`](crate::error::GetCallAnalyticsJobError)
    pub fn get_call_analytics_job(&self) -> crate::client::fluent_builders::GetCallAnalyticsJob {
                                crate::client::fluent_builders::GetCallAnalyticsJob::new(self.handle.clone())
                            }
}

