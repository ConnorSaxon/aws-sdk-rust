// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeReceiptRule`](crate::client::fluent_builders::DescribeReceiptRule) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rule_set_name(impl Into<String>)`](crate::client::fluent_builders::DescribeReceiptRule::rule_set_name) / [`set_rule_set_name(Option<String>)`](crate::client::fluent_builders::DescribeReceiptRule::set_rule_set_name): <p>The name of the receipt rule set that the receipt rule belongs to.</p>
    ///   - [`rule_name(impl Into<String>)`](crate::client::fluent_builders::DescribeReceiptRule::rule_name) / [`set_rule_name(Option<String>)`](crate::client::fluent_builders::DescribeReceiptRule::set_rule_name): <p>The name of the receipt rule.</p>
                            /// - On success, responds with [`DescribeReceiptRuleOutput`](crate::output::DescribeReceiptRuleOutput) with field(s):
    ///   - [`rule(Option<ReceiptRule>)`](crate::output::DescribeReceiptRuleOutput::rule): <p>A data structure that contains the specified receipt rule's name, actions, recipients, domains, enabled status, scan status, and Transport Layer Security (TLS) policy.</p>
                            /// - On failure, responds with [`SdkError<DescribeReceiptRuleError>`](crate::error::DescribeReceiptRuleError)
    pub fn describe_receipt_rule(&self) -> crate::client::fluent_builders::DescribeReceiptRule {
                                crate::client::fluent_builders::DescribeReceiptRule::new(self.handle.clone())
                            }
}

