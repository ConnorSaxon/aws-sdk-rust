// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRateBasedStatementManagedKeys`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`scope(Scope)`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys::scope) / [`set_scope(Option<Scope>)`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys::set_scope): <p>Specifies whether this is for an Amazon CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB), an Amazon API Gateway REST API, an AppSync GraphQL API, or an Amazon Cognito user pool. </p>  <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>  <ul>   <li> <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li>   <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li>  </ul>
    ///   - [`web_acl_name(impl Into<String>)`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys::web_acl_name) / [`set_web_acl_name(Option<String>)`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys::set_web_acl_name): <p>The name of the web ACL. You cannot change the name of a web ACL after you create it.</p>
    ///   - [`web_acl_id(impl Into<String>)`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys::web_acl_id) / [`set_web_acl_id(Option<String>)`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys::set_web_acl_id): <p>The unique identifier for the web ACL. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    ///   - [`rule_group_rule_name(impl Into<String>)`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys::rule_group_rule_name) / [`set_rule_group_rule_name(Option<String>)`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys::set_rule_group_rule_name): <p>The name of the rule group reference statement in your web ACL. This is required only when you have the rate-based rule nested inside a rule group. </p>
    ///   - [`rule_name(impl Into<String>)`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys::rule_name) / [`set_rule_name(Option<String>)`](crate::client::fluent_builders::GetRateBasedStatementManagedKeys::set_rule_name): <p>The name of the rate-based rule to get the keys for. If you have the rule defined inside a rule group that you're using in your web ACL, also provide the name of the rule group reference statement in the request parameter <code>RuleGroupRuleName</code>.</p>
                            /// - On success, responds with [`GetRateBasedStatementManagedKeysOutput`](crate::output::GetRateBasedStatementManagedKeysOutput) with field(s):
    ///   - [`managed_keys_ipv4(Option<RateBasedStatementManagedKeysIpSet>)`](crate::output::GetRateBasedStatementManagedKeysOutput::managed_keys_ipv4): <p>The keys that are of Internet Protocol version 4 (IPv4). </p>
    ///   - [`managed_keys_ipv6(Option<RateBasedStatementManagedKeysIpSet>)`](crate::output::GetRateBasedStatementManagedKeysOutput::managed_keys_ipv6): <p>The keys that are of Internet Protocol version 6 (IPv6). </p>
                            /// - On failure, responds with [`SdkError<GetRateBasedStatementManagedKeysError>`](crate::error::GetRateBasedStatementManagedKeysError)
    pub fn get_rate_based_statement_managed_keys(&self) -> crate::client::fluent_builders::GetRateBasedStatementManagedKeys {
                                crate::client::fluent_builders::GetRateBasedStatementManagedKeys::new(self.handle.clone())
                            }
}

