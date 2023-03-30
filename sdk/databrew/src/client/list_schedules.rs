// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListSchedules`](crate::client::fluent_builders::ListSchedules) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListSchedules::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_name(impl Into<String>)`](crate::client::fluent_builders::ListSchedules::job_name) / [`set_job_name(Option<String>)`](crate::client::fluent_builders::ListSchedules::set_job_name): <p>The name of the job that these schedules apply to.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListSchedules::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListSchedules::set_max_results): <p>The maximum number of results to return in this request. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListSchedules::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListSchedules::set_next_token): <p>The token returned by a previous call to retrieve the next set of results.</p>
                            /// - On success, responds with [`ListSchedulesOutput`](crate::output::ListSchedulesOutput) with field(s):
    ///   - [`schedules(Option<Vec<Schedule>>)`](crate::output::ListSchedulesOutput::schedules): <p>A list of schedules that are defined.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListSchedulesOutput::next_token): <p>A token that you can use in a subsequent call to retrieve the next set of results.</p>
                            /// - On failure, responds with [`SdkError<ListSchedulesError>`](crate::error::ListSchedulesError)
    pub fn list_schedules(&self) -> crate::client::fluent_builders::ListSchedules {
                                crate::client::fluent_builders::ListSchedules::new(self.handle.clone())
                            }
}

