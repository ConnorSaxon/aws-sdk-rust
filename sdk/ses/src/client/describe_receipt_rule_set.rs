// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeReceiptRuleSet`](crate::client::fluent_builders::DescribeReceiptRuleSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rule_set_name(impl Into<String>)`](crate::client::fluent_builders::DescribeReceiptRuleSet::rule_set_name) / [`set_rule_set_name(Option<String>)`](crate::client::fluent_builders::DescribeReceiptRuleSet::set_rule_set_name): <p>The name of the receipt rule set to describe.</p>
                            /// - On success, responds with [`DescribeReceiptRuleSetOutput`](crate::output::DescribeReceiptRuleSetOutput) with field(s):
    ///   - [`metadata(Option<ReceiptRuleSetMetadata>)`](crate::output::DescribeReceiptRuleSetOutput::metadata): <p>The metadata for the receipt rule set, which consists of the rule set name and the timestamp of when the rule set was created.</p>
    ///   - [`rules(Option<Vec<ReceiptRule>>)`](crate::output::DescribeReceiptRuleSetOutput::rules): <p>A list of the receipt rules that belong to the specified receipt rule set.</p>
                            /// - On failure, responds with [`SdkError<DescribeReceiptRuleSetError>`](crate::error::DescribeReceiptRuleSetError)
    pub fn describe_receipt_rule_set(&self) -> crate::client::fluent_builders::DescribeReceiptRuleSet {
                                crate::client::fluent_builders::DescribeReceiptRuleSet::new(self.handle.clone())
                            }
}

