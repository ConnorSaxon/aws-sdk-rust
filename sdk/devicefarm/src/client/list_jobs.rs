// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListJobs`](crate::client::fluent_builders::ListJobs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListJobs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::ListJobs::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::ListJobs::set_arn): <p>The run's Amazon Resource Name (ARN).</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListJobs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListJobs::set_next_token): <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
                            /// - On success, responds with [`ListJobsOutput`](crate::output::ListJobsOutput) with field(s):
    ///   - [`jobs(Option<Vec<Job>>)`](crate::output::ListJobsOutput::jobs): <p>Information about the jobs.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListJobsOutput::next_token): <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
                            /// - On failure, responds with [`SdkError<ListJobsError>`](crate::error::ListJobsError)
    pub fn list_jobs(&self) -> crate::client::fluent_builders::ListJobs {
                                crate::client::fluent_builders::ListJobs::new(self.handle.clone())
                            }
}

