// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateXssMatchSet`](crate::client::fluent_builders::CreateXssMatchSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateXssMatchSet::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateXssMatchSet::set_name): <p>A friendly name or description for the <code>XssMatchSet</code> that you're creating. You can't change <code>Name</code> after you create the <code>XssMatchSet</code>.</p>
    ///   - [`change_token(impl Into<String>)`](crate::client::fluent_builders::CreateXssMatchSet::change_token) / [`set_change_token(Option<String>)`](crate::client::fluent_builders::CreateXssMatchSet::set_change_token): <p>The value returned by the most recent call to <code>GetChangeToken</code>.</p>
                            /// - On success, responds with [`CreateXssMatchSetOutput`](crate::output::CreateXssMatchSetOutput) with field(s):
    ///   - [`xss_match_set(Option<XssMatchSet>)`](crate::output::CreateXssMatchSetOutput::xss_match_set): <p>An <code>XssMatchSet</code>.</p>
    ///   - [`change_token(Option<String>)`](crate::output::CreateXssMatchSetOutput::change_token): <p>The <code>ChangeToken</code> that you used to submit the <code>CreateXssMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <code>GetChangeTokenStatus</code>.</p>
                            /// - On failure, responds with [`SdkError<CreateXssMatchSetError>`](crate::error::CreateXssMatchSetError)
    pub fn create_xss_match_set(&self) -> crate::client::fluent_builders::CreateXssMatchSet {
                                crate::client::fluent_builders::CreateXssMatchSet::new(self.handle.clone())
                            }
}

