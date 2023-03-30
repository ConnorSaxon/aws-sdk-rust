// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteInsightRules`](crate::client::fluent_builders::DeleteInsightRules) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rule_names(Vec<String>)`](crate::client::fluent_builders::DeleteInsightRules::rule_names) / [`set_rule_names(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteInsightRules::set_rule_names): <p>An array of the rule names to delete. If you need to find out the names of your rules, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeInsightRules.html">DescribeInsightRules</a>.</p>
                            /// - On success, responds with [`DeleteInsightRulesOutput`](crate::output::DeleteInsightRulesOutput) with field(s):
    ///   - [`failures(Option<Vec<PartialFailure>>)`](crate::output::DeleteInsightRulesOutput::failures): <p>An array listing the rules that could not be deleted. You cannot delete built-in rules.</p>
                            /// - On failure, responds with [`SdkError<DeleteInsightRulesError>`](crate::error::DeleteInsightRulesError)
    pub fn delete_insight_rules(&self) -> crate::client::fluent_builders::DeleteInsightRules {
                                crate::client::fluent_builders::DeleteInsightRules::new(self.handle.clone())
                            }
}

