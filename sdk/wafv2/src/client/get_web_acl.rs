// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetWebACL`](crate::client::fluent_builders::GetWebACL) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::GetWebACL::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::GetWebACL::set_name): <p>The name of the web ACL. You cannot change the name of a web ACL after you create it.</p>
    ///   - [`scope(Scope)`](crate::client::fluent_builders::GetWebACL::scope) / [`set_scope(Option<Scope>)`](crate::client::fluent_builders::GetWebACL::set_scope): <p>Specifies whether this is for an Amazon CloudFront distribution or for a regional application. A regional application can be an Application Load Balancer (ALB), an Amazon API Gateway REST API, an AppSync GraphQL API, or an Amazon Cognito user pool. </p>  <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>  <ul>   <li> <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p> </li>   <li> <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p> </li>  </ul>
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetWebACL::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetWebACL::set_id): <p>The unique identifier for the web ACL. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
                            /// - On success, responds with [`GetWebAclOutput`](crate::output::GetWebAclOutput) with field(s):
    ///   - [`web_acl(Option<WebAcl>)`](crate::output::GetWebAclOutput::web_acl): <p>The web ACL specification. You can modify the settings in this web ACL and use it to update this web ACL or create a new one.</p>
    ///   - [`lock_token(Option<String>)`](crate::output::GetWebAclOutput::lock_token): <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation. </p>
    ///   - [`application_integration_url(Option<String>)`](crate::output::GetWebAclOutput::application_integration_url): <p>The URL to use in SDK integrations with Amazon Web Services managed rule groups. For example, you can use the integration SDKs with the account takeover prevention managed rule group <code>AWSManagedRulesATPRuleSet</code>. This is only populated if you are using a rule group in your web ACL that integrates with your applications in this way. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-application-integration.html">WAF client application integration</a> in the <i>WAF Developer Guide</i>.</p>
                            /// - On failure, responds with [`SdkError<GetWebACLError>`](crate::error::GetWebACLError)
    pub fn get_web_acl(&self) -> crate::client::fluent_builders::GetWebACL {
                                crate::client::fluent_builders::GetWebACL::new(self.handle.clone())
                            }
}

