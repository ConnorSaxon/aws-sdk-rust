// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteConfiguredTableAnalysisRule`](crate::client::fluent_builders::DeleteConfiguredTableAnalysisRule) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configured_table_identifier(impl Into<String>)`](crate::client::fluent_builders::DeleteConfiguredTableAnalysisRule::configured_table_identifier) / [`set_configured_table_identifier(Option<String>)`](crate::client::fluent_builders::DeleteConfiguredTableAnalysisRule::set_configured_table_identifier): <p>The unique identifier for the configured table that the analysis rule applies to. Currently accepts the configured table ID.</p>
    ///   - [`analysis_rule_type(ConfiguredTableAnalysisRuleType)`](crate::client::fluent_builders::DeleteConfiguredTableAnalysisRule::analysis_rule_type) / [`set_analysis_rule_type(Option<ConfiguredTableAnalysisRuleType>)`](crate::client::fluent_builders::DeleteConfiguredTableAnalysisRule::set_analysis_rule_type): <p>The analysis rule type to be deleted. Configured table analysis rules are uniquely identified by their configured table identifier and analysis rule type.</p>
                            /// - On success, responds with [`DeleteConfiguredTableAnalysisRuleOutput`](crate::output::DeleteConfiguredTableAnalysisRuleOutput)
                            /// - On failure, responds with [`SdkError<DeleteConfiguredTableAnalysisRuleError>`](crate::error::DeleteConfiguredTableAnalysisRuleError)
    pub fn delete_configured_table_analysis_rule(&self) -> crate::client::fluent_builders::DeleteConfiguredTableAnalysisRule {
                                crate::client::fluent_builders::DeleteConfiguredTableAnalysisRule::new(self.handle.clone())
                            }
}

