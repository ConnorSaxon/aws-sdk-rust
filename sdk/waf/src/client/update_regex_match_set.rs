// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateRegexMatchSet`](crate::client::fluent_builders::UpdateRegexMatchSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`regex_match_set_id(impl Into<String>)`](crate::client::fluent_builders::UpdateRegexMatchSet::regex_match_set_id) / [`set_regex_match_set_id(Option<String>)`](crate::client::fluent_builders::UpdateRegexMatchSet::set_regex_match_set_id): <p>The <code>RegexMatchSetId</code> of the <code>RegexMatchSet</code> that you want to update. <code>RegexMatchSetId</code> is returned by <code>CreateRegexMatchSet</code> and by <code>ListRegexMatchSets</code>.</p>
    ///   - [`updates(Vec<RegexMatchSetUpdate>)`](crate::client::fluent_builders::UpdateRegexMatchSet::updates) / [`set_updates(Option<Vec<RegexMatchSetUpdate>>)`](crate::client::fluent_builders::UpdateRegexMatchSet::set_updates): <p>An array of <code>RegexMatchSetUpdate</code> objects that you want to insert into or delete from a <code>RegexMatchSet</code>. For more information, see <code>RegexMatchTuple</code>.</p>
    ///   - [`change_token(impl Into<String>)`](crate::client::fluent_builders::UpdateRegexMatchSet::change_token) / [`set_change_token(Option<String>)`](crate::client::fluent_builders::UpdateRegexMatchSet::set_change_token): <p>The value returned by the most recent call to <code>GetChangeToken</code>.</p>
                            /// - On success, responds with [`UpdateRegexMatchSetOutput`](crate::output::UpdateRegexMatchSetOutput) with field(s):
    ///   - [`change_token(Option<String>)`](crate::output::UpdateRegexMatchSetOutput::change_token): <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRegexMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <code>GetChangeTokenStatus</code>.</p>
                            /// - On failure, responds with [`SdkError<UpdateRegexMatchSetError>`](crate::error::UpdateRegexMatchSetError)
    pub fn update_regex_match_set(&self) -> crate::client::fluent_builders::UpdateRegexMatchSet {
                                crate::client::fluent_builders::UpdateRegexMatchSet::new(self.handle.clone())
                            }
}

