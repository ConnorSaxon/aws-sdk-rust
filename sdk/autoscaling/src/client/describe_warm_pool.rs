// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeWarmPool`](crate::client::fluent_builders::DescribeWarmPool) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`auto_scaling_group_name(impl Into<String>)`](crate::client::fluent_builders::DescribeWarmPool::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::client::fluent_builders::DescribeWarmPool::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeWarmPool::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeWarmPool::set_max_records): <p>The maximum number of instances to return with this call. The maximum value is <code>50</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeWarmPool::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeWarmPool::set_next_token): <p>The token for the next set of instances to return. (You received this token from a previous call.)</p>
                            /// - On success, responds with [`DescribeWarmPoolOutput`](crate::output::DescribeWarmPoolOutput) with field(s):
    ///   - [`warm_pool_configuration(Option<WarmPoolConfiguration>)`](crate::output::DescribeWarmPoolOutput::warm_pool_configuration): <p>The warm pool configuration details. </p>
    ///   - [`instances(Option<Vec<Instance>>)`](crate::output::DescribeWarmPoolOutput::instances): <p>The instances that are currently in the warm pool.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeWarmPoolOutput::next_token): <p>This string indicates that the response contains more items than can be returned in a single response. To receive additional items, specify this string for the <code>NextToken</code> value when requesting the next set of items. This value is null when there are no more items to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeWarmPoolError>`](crate::error::DescribeWarmPoolError)
    pub fn describe_warm_pool(&self) -> crate::client::fluent_builders::DescribeWarmPool {
                                crate::client::fluent_builders::DescribeWarmPool::new(self.handle.clone())
                            }
}

