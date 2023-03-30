// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListApps`](crate::client::fluent_builders::ListApps) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_ids(Vec<String>)`](crate::client::fluent_builders::ListApps::app_ids) / [`set_app_ids(Option<Vec<String>>)`](crate::client::fluent_builders::ListApps::set_app_ids): <p>The unique application IDs.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListApps::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListApps::set_next_token): <p>The token for the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListApps::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListApps::set_max_results): <p>The maximum number of results to return in a single call. The default value is 100. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value. </p>
                            /// - On success, responds with [`ListAppsOutput`](crate::output::ListAppsOutput) with field(s):
    ///   - [`apps(Option<Vec<AppSummary>>)`](crate::output::ListAppsOutput::apps): <p>The application summaries.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListAppsOutput::next_token): <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<ListAppsError>`](crate::error::ListAppsError)
    pub fn list_apps(&self) -> crate::client::fluent_builders::ListApps {
                                crate::client::fluent_builders::ListApps::new(self.handle.clone())
                            }
}

