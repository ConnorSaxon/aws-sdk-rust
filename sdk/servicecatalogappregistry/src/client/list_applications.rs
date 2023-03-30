// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListApplications`](crate::client::fluent_builders::ListApplications) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListApplications::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListApplications::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListApplications::set_next_token): <p>The token to use to get the next page of results after a previous API call. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListApplications::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListApplications::set_max_results): <p>The upper bound of the number of results to return (cannot exceed 25). If this parameter is omitted, it defaults to 25. This value is optional.</p>
                            /// - On success, responds with [`ListApplicationsOutput`](crate::output::ListApplicationsOutput) with field(s):
    ///   - [`applications(Option<Vec<ApplicationSummary>>)`](crate::output::ListApplicationsOutput::applications): <p>This list of applications.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListApplicationsOutput::next_token): <p>The token to use to get the next page of results after a previous API call. </p>
                            /// - On failure, responds with [`SdkError<ListApplicationsError>`](crate::error::ListApplicationsError)
    pub fn list_applications(&self) -> crate::client::fluent_builders::ListApplications {
                                crate::client::fluent_builders::ListApplications::new(self.handle.clone())
                            }
}

