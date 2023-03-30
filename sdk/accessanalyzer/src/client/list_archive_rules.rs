// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListArchiveRules`](crate::client::fluent_builders::ListArchiveRules) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListArchiveRules::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`analyzer_name(impl Into<String>)`](crate::client::fluent_builders::ListArchiveRules::analyzer_name) / [`set_analyzer_name(Option<String>)`](crate::client::fluent_builders::ListArchiveRules::set_analyzer_name): <p>The name of the analyzer to retrieve rules from.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListArchiveRules::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListArchiveRules::set_next_token): <p>A token used for pagination of results returned.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListArchiveRules::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListArchiveRules::set_max_results): <p>The maximum number of results to return in the request.</p>
                            /// - On success, responds with [`ListArchiveRulesOutput`](crate::output::ListArchiveRulesOutput) with field(s):
    ///   - [`archive_rules(Option<Vec<ArchiveRuleSummary>>)`](crate::output::ListArchiveRulesOutput::archive_rules): <p>A list of archive rules created for the specified analyzer.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListArchiveRulesOutput::next_token): <p>A token used for pagination of results returned.</p>
                            /// - On failure, responds with [`SdkError<ListArchiveRulesError>`](crate::error::ListArchiveRulesError)
    pub fn list_archive_rules(&self) -> crate::client::fluent_builders::ListArchiveRules {
                                crate::client::fluent_builders::ListArchiveRules::new(self.handle.clone())
                            }
}

