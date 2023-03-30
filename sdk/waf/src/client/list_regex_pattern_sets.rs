// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListRegexPatternSets`](crate::client::fluent_builders::ListRegexPatternSets) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_marker(impl Into<String>)`](crate::client::fluent_builders::ListRegexPatternSets::next_marker) / [`set_next_marker(Option<String>)`](crate::client::fluent_builders::ListRegexPatternSets::set_next_marker): <p>If you specify a value for <code>Limit</code> and you have more <code>RegexPatternSet</code> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>RegexPatternSet</code> objects. For the second and subsequent <code>ListRegexPatternSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RegexPatternSet</code> objects.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListRegexPatternSets::limit) / [`set_limit(i32)`](crate::client::fluent_builders::ListRegexPatternSets::set_limit): <p>Specifies the number of <code>RegexPatternSet</code> objects that you want AWS WAF to return for this request. If you have more <code>RegexPatternSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RegexPatternSet</code> objects.</p>
                            /// - On success, responds with [`ListRegexPatternSetsOutput`](crate::output::ListRegexPatternSetsOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::output::ListRegexPatternSetsOutput::next_marker): <p>If you have more <code>RegexPatternSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>RegexPatternSet</code> objects, submit another <code>ListRegexPatternSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    ///   - [`regex_pattern_sets(Option<Vec<RegexPatternSetSummary>>)`](crate::output::ListRegexPatternSetsOutput::regex_pattern_sets): <p>An array of <code>RegexPatternSetSummary</code> objects.</p>
                            /// - On failure, responds with [`SdkError<ListRegexPatternSetsError>`](crate::error::ListRegexPatternSetsError)
    pub fn list_regex_pattern_sets(&self) -> crate::client::fluent_builders::ListRegexPatternSets {
                                crate::client::fluent_builders::ListRegexPatternSets::new(self.handle.clone())
                            }
}

