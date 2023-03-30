// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeLoadBalancerAttributes`](crate::client::fluent_builders::DescribeLoadBalancerAttributes) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`load_balancer_name(impl Into<String>)`](crate::client::fluent_builders::DescribeLoadBalancerAttributes::load_balancer_name) / [`set_load_balancer_name(Option<String>)`](crate::client::fluent_builders::DescribeLoadBalancerAttributes::set_load_balancer_name): <p>The name of the load balancer.</p>
                            /// - On success, responds with [`DescribeLoadBalancerAttributesOutput`](crate::output::DescribeLoadBalancerAttributesOutput) with field(s):
    ///   - [`load_balancer_attributes(Option<LoadBalancerAttributes>)`](crate::output::DescribeLoadBalancerAttributesOutput::load_balancer_attributes): <p>Information about the load balancer attributes.</p>
                            /// - On failure, responds with [`SdkError<DescribeLoadBalancerAttributesError>`](crate::error::DescribeLoadBalancerAttributesError)
    pub fn describe_load_balancer_attributes(&self) -> crate::client::fluent_builders::DescribeLoadBalancerAttributes {
                                crate::client::fluent_builders::DescribeLoadBalancerAttributes::new(self.handle.clone())
                            }
}

