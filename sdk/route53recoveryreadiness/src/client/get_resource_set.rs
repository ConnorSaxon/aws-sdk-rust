// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetResourceSet`](crate::client::fluent_builders::GetResourceSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_set_name(impl Into<String>)`](crate::client::fluent_builders::GetResourceSet::resource_set_name) / [`set_resource_set_name(Option<String>)`](crate::client::fluent_builders::GetResourceSet::set_resource_set_name): <p>Name of a resource set.</p>
                            /// - On success, responds with [`GetResourceSetOutput`](crate::output::GetResourceSetOutput) with field(s):
    ///   - [`resource_set_arn(Option<String>)`](crate::output::GetResourceSetOutput::resource_set_arn): <p>The Amazon Resource Name (ARN) for the resource set.</p>
    ///   - [`resource_set_name(Option<String>)`](crate::output::GetResourceSetOutput::resource_set_name): <p>The name of the resource set.</p>
    ///   - [`resource_set_type(Option<String>)`](crate::output::GetResourceSetOutput::resource_set_type): <p>The resource type of the resources in the resource set. Enter one of the following values for resource type:</p>  <p>AWS::ApiGateway::Stage, AWS::ApiGatewayV2::Stage, AWS::AutoScaling::AutoScalingGroup, AWS::CloudWatch::Alarm, AWS::EC2::CustomerGateway, AWS::DynamoDB::Table, AWS::EC2::Volume, AWS::ElasticLoadBalancing::LoadBalancer, AWS::ElasticLoadBalancingV2::LoadBalancer, AWS::Lambda::Function, AWS::MSK::Cluster, AWS::RDS::DBCluster, AWS::Route53::HealthCheck, AWS::SQS::Queue, AWS::SNS::Topic, AWS::SNS::Subscription, AWS::EC2::VPC, AWS::EC2::VPNConnection, AWS::EC2::VPNGateway, AWS::Route53RecoveryReadiness::DNSTargetResource</p>
    ///   - [`resources(Option<Vec<Resource>>)`](crate::output::GetResourceSetOutput::resources): <p>A list of resource objects.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::GetResourceSetOutput::tags): <p>A collection of tags associated with a resource.</p>
                            /// - On failure, responds with [`SdkError<GetResourceSetError>`](crate::error::GetResourceSetError)
    pub fn get_resource_set(&self) -> crate::client::fluent_builders::GetResourceSet {
                                crate::client::fluent_builders::GetResourceSet::new(self.handle.clone())
                            }
}

