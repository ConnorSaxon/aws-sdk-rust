// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRateBasedRuleManagedKeys`](crate::client::fluent_builders::GetRateBasedRuleManagedKeys) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rule_id(impl Into<String>)`](crate::client::fluent_builders::GetRateBasedRuleManagedKeys::rule_id) / [`set_rule_id(Option<String>)`](crate::client::fluent_builders::GetRateBasedRuleManagedKeys::set_rule_id): <p>The <code>RuleId</code> of the <code>RateBasedRule</code> for which you want to get a list of <code>ManagedKeys</code>. <code>RuleId</code> is returned by <code>CreateRateBasedRule</code> and by <code>ListRateBasedRules</code>.</p>
    ///   - [`next_marker(impl Into<String>)`](crate::client::fluent_builders::GetRateBasedRuleManagedKeys::next_marker) / [`set_next_marker(Option<String>)`](crate::client::fluent_builders::GetRateBasedRuleManagedKeys::set_next_marker): <p>A null value and not currently used. Do not include this in your request.</p>
                            /// - On success, responds with [`GetRateBasedRuleManagedKeysOutput`](crate::output::GetRateBasedRuleManagedKeysOutput) with field(s):
    ///   - [`managed_keys(Option<Vec<String>>)`](crate::output::GetRateBasedRuleManagedKeysOutput::managed_keys): <p>An array of IP addresses that currently are blocked by the specified <code>RateBasedRule</code>. </p>
    ///   - [`next_marker(Option<String>)`](crate::output::GetRateBasedRuleManagedKeysOutput::next_marker): <p>A null value and not currently used.</p>
                            /// - On failure, responds with [`SdkError<GetRateBasedRuleManagedKeysError>`](crate::error::GetRateBasedRuleManagedKeysError)
    pub fn get_rate_based_rule_managed_keys(&self) -> crate::client::fluent_builders::GetRateBasedRuleManagedKeys {
                                crate::client::fluent_builders::GetRateBasedRuleManagedKeys::new(self.handle.clone())
                            }
}

