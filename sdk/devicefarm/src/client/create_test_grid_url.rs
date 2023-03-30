// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateTestGridUrl`](crate::client::fluent_builders::CreateTestGridUrl) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project_arn(impl Into<String>)`](crate::client::fluent_builders::CreateTestGridUrl::project_arn) / [`set_project_arn(Option<String>)`](crate::client::fluent_builders::CreateTestGridUrl::set_project_arn): <p>ARN (from <code>CreateTestGridProject</code> or <code>ListTestGridProjects</code>) to associate with the short-term URL. </p>
    ///   - [`expires_in_seconds(i32)`](crate::client::fluent_builders::CreateTestGridUrl::expires_in_seconds) / [`set_expires_in_seconds(Option<i32>)`](crate::client::fluent_builders::CreateTestGridUrl::set_expires_in_seconds): <p>Lifetime, in seconds, of the URL.</p>
                            /// - On success, responds with [`CreateTestGridUrlOutput`](crate::output::CreateTestGridUrlOutput) with field(s):
    ///   - [`url(Option<String>)`](crate::output::CreateTestGridUrlOutput::url): <p>A signed URL, expiring in <code>CreateTestGridUrlRequest$expiresInSeconds</code> seconds, to be passed to a <code>RemoteWebDriver</code>. </p>
    ///   - [`expires(Option<DateTime>)`](crate::output::CreateTestGridUrlOutput::expires): <p>The number of seconds the URL from <code>CreateTestGridUrlResult$url</code> stays active.</p>
                            /// - On failure, responds with [`SdkError<CreateTestGridUrlError>`](crate::error::CreateTestGridUrlError)
    pub fn create_test_grid_url(&self) -> crate::client::fluent_builders::CreateTestGridUrl {
                                crate::client::fluent_builders::CreateTestGridUrl::new(self.handle.clone())
                            }
}

