// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteRule`](crate::client::fluent_builders::DeleteRule) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeleteRule::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeleteRule::set_name): <p>The name of the rule.</p>
    ///   - [`event_bus_name(impl Into<String>)`](crate::client::fluent_builders::DeleteRule::event_bus_name) / [`set_event_bus_name(Option<String>)`](crate::client::fluent_builders::DeleteRule::set_event_bus_name): <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    ///   - [`force(bool)`](crate::client::fluent_builders::DeleteRule::force) / [`set_force(bool)`](crate::client::fluent_builders::DeleteRule::set_force): <p>If this is a managed rule, created by an Amazon Web Services service on your behalf, you must specify <code>Force</code> as <code>True</code> to delete the rule. This parameter is ignored for rules that are not managed rules. You can check whether a rule is a managed rule by using <code>DescribeRule</code> or <code>ListRules</code> and checking the <code>ManagedBy</code> field of the response.</p>
                            /// - On success, responds with [`DeleteRuleOutput`](crate::output::DeleteRuleOutput)
                            /// - On failure, responds with [`SdkError<DeleteRuleError>`](crate::error::DeleteRuleError)
    pub fn delete_rule(&self) -> crate::client::fluent_builders::DeleteRule {
                                crate::client::fluent_builders::DeleteRule::new(self.handle.clone())
                            }
}

