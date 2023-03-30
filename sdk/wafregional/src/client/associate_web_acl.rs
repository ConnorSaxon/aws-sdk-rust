// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateWebACL`](crate::client::fluent_builders::AssociateWebACL) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`web_acl_id(impl Into<String>)`](crate::client::fluent_builders::AssociateWebACL::web_acl_id) / [`set_web_acl_id(Option<String>)`](crate::client::fluent_builders::AssociateWebACL::set_web_acl_id): <p>A unique identifier (ID) for the web ACL. </p>
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::AssociateWebACL::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::AssociateWebACL::set_resource_arn): <p>The ARN (Amazon Resource Name) of the resource to be protected, either an application load balancer or Amazon API Gateway stage. </p>  <p>The ARN should be in one of the following formats:</p>  <ul>   <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> </p> </li>   <li> <p>For an Amazon API Gateway stage: <code>arn:aws:apigateway:<i>region</i>::/restapis/<i>api-id</i>/stages/<i>stage-name</i> </code> </p> </li>  </ul>
                            /// - On success, responds with [`AssociateWebAclOutput`](crate::output::AssociateWebAclOutput)
                            /// - On failure, responds with [`SdkError<AssociateWebACLError>`](crate::error::AssociateWebACLError)
    pub fn associate_web_acl(&self) -> crate::client::fluent_builders::AssociateWebACL {
                                crate::client::fluent_builders::AssociateWebACL::new(self.handle.clone())
                            }
}

