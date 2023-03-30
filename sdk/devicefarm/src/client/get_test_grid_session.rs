// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetTestGridSession`](crate::client::fluent_builders::GetTestGridSession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project_arn(impl Into<String>)`](crate::client::fluent_builders::GetTestGridSession::project_arn) / [`set_project_arn(Option<String>)`](crate::client::fluent_builders::GetTestGridSession::set_project_arn): <p>The ARN for the project that this session belongs to. See <code>CreateTestGridProject</code> and <code>ListTestGridProjects</code>.</p>
    ///   - [`session_id(impl Into<String>)`](crate::client::fluent_builders::GetTestGridSession::session_id) / [`set_session_id(Option<String>)`](crate::client::fluent_builders::GetTestGridSession::set_session_id): <p>An ID associated with this session.</p>
    ///   - [`session_arn(impl Into<String>)`](crate::client::fluent_builders::GetTestGridSession::session_arn) / [`set_session_arn(Option<String>)`](crate::client::fluent_builders::GetTestGridSession::set_session_arn): <p>An ARN that uniquely identifies a <code>TestGridSession</code>.</p>
                            /// - On success, responds with [`GetTestGridSessionOutput`](crate::output::GetTestGridSessionOutput) with field(s):
    ///   - [`test_grid_session(Option<TestGridSession>)`](crate::output::GetTestGridSessionOutput::test_grid_session): <p>The <code>TestGridSession</code> that was requested.</p>
                            /// - On failure, responds with [`SdkError<GetTestGridSessionError>`](crate::error::GetTestGridSessionError)
    pub fn get_test_grid_session(&self) -> crate::client::fluent_builders::GetTestGridSession {
                                crate::client::fluent_builders::GetTestGridSession::new(self.handle.clone())
                            }
}

