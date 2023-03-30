// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RevokeIpRules`](crate::client::fluent_builders::RevokeIpRules) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`group_id(impl Into<String>)`](crate::client::fluent_builders::RevokeIpRules::group_id) / [`set_group_id(Option<String>)`](crate::client::fluent_builders::RevokeIpRules::set_group_id): <p>The identifier of the group.</p>
    ///   - [`user_rules(Vec<String>)`](crate::client::fluent_builders::RevokeIpRules::user_rules) / [`set_user_rules(Option<Vec<String>>)`](crate::client::fluent_builders::RevokeIpRules::set_user_rules): <p>The rules to remove from the group.</p>
                            /// - On success, responds with [`RevokeIpRulesOutput`](crate::output::RevokeIpRulesOutput)
                            /// - On failure, responds with [`SdkError<RevokeIpRulesError>`](crate::error::RevokeIpRulesError)
    pub fn revoke_ip_rules(&self) -> crate::client::fluent_builders::RevokeIpRules {
                                crate::client::fluent_builders::RevokeIpRules::new(self.handle.clone())
                            }
}

