// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateVpcLink`](crate::client::fluent_builders::CreateVpcLink) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateVpcLink::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateVpcLink::set_name): <p>The name used to label and identify the VPC link.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateVpcLink::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateVpcLink::set_description): <p>The description of the VPC link.</p>
    ///   - [`target_arns(Vec<String>)`](crate::client::fluent_builders::CreateVpcLink::target_arns) / [`set_target_arns(Option<Vec<String>>)`](crate::client::fluent_builders::CreateVpcLink::set_target_arns): <p>The ARN of the network load balancer of the VPC targeted by the VPC link. The network load balancer must be owned by the same AWS account of the API owner.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateVpcLink::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateVpcLink::set_tags): <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
                            /// - On success, responds with [`CreateVpcLinkOutput`](crate::output::CreateVpcLinkOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::CreateVpcLinkOutput::id): <p>The identifier of the VpcLink. It is used in an Integration to reference this VpcLink.</p>
    ///   - [`name(Option<String>)`](crate::output::CreateVpcLinkOutput::name): <p>The name used to label and identify the VPC link.</p>
    ///   - [`description(Option<String>)`](crate::output::CreateVpcLinkOutput::description): <p>The description of the VPC link.</p>
    ///   - [`target_arns(Option<Vec<String>>)`](crate::output::CreateVpcLinkOutput::target_arns): <p>The ARN of the network load balancer of the VPC targeted by the VPC link. The network load balancer must be owned by the same AWS account of the API owner.</p>
    ///   - [`status(Option<VpcLinkStatus>)`](crate::output::CreateVpcLinkOutput::status): <p>The status of the VPC link. The valid values are <code>AVAILABLE</code>, <code>PENDING</code>, <code>DELETING</code>, or <code>FAILED</code>. Deploying an API will wait if the status is <code>PENDING</code> and will fail if the status is <code>DELETING</code>. </p>
    ///   - [`status_message(Option<String>)`](crate::output::CreateVpcLinkOutput::status_message): <p>A description about the VPC link status.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::CreateVpcLinkOutput::tags): <p>The collection of tags. Each tag element is associated with a given resource.</p>
                            /// - On failure, responds with [`SdkError<CreateVpcLinkError>`](crate::error::CreateVpcLinkError)
    pub fn create_vpc_link(&self) -> crate::client::fluent_builders::CreateVpcLink {
                                crate::client::fluent_builders::CreateVpcLink::new(self.handle.clone())
                            }
}

