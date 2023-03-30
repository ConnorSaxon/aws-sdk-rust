// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeLoadBalancers`](crate::client::fluent_builders::DescribeLoadBalancers) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeLoadBalancers::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`load_balancer_arns(Vec<String>)`](crate::client::fluent_builders::DescribeLoadBalancers::load_balancer_arns) / [`set_load_balancer_arns(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeLoadBalancers::set_load_balancer_arns): <p>The Amazon Resource Names (ARN) of the load balancers. You can specify up to 20 load balancers in a single call.</p>
    ///   - [`names(Vec<String>)`](crate::client::fluent_builders::DescribeLoadBalancers::names) / [`set_names(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeLoadBalancers::set_names): <p>The names of the load balancers.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeLoadBalancers::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeLoadBalancers::set_marker): <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    ///   - [`page_size(i32)`](crate::client::fluent_builders::DescribeLoadBalancers::page_size) / [`set_page_size(Option<i32>)`](crate::client::fluent_builders::DescribeLoadBalancers::set_page_size): <p>The maximum number of results to return with this call.</p>
                            /// - On success, responds with [`DescribeLoadBalancersOutput`](crate::output::DescribeLoadBalancersOutput) with field(s):
    ///   - [`load_balancers(Option<Vec<LoadBalancer>>)`](crate::output::DescribeLoadBalancersOutput::load_balancers): <p>Information about the load balancers.</p>
    ///   - [`next_marker(Option<String>)`](crate::output::DescribeLoadBalancersOutput::next_marker): <p>If there are additional results, this is the marker for the next set of results. Otherwise, this is null.</p>
                            /// - On failure, responds with [`SdkError<DescribeLoadBalancersError>`](crate::error::DescribeLoadBalancersError)
    pub fn describe_load_balancers(&self) -> crate::client::fluent_builders::DescribeLoadBalancers {
                                crate::client::fluent_builders::DescribeLoadBalancers::new(self.handle.clone())
                            }
}

