// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeTimeBasedAutoScaling`](crate::client::fluent_builders::DescribeTimeBasedAutoScaling) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_ids(Vec<String>)`](crate::client::fluent_builders::DescribeTimeBasedAutoScaling::instance_ids) / [`set_instance_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeTimeBasedAutoScaling::set_instance_ids): <p>An array of instance IDs.</p>
                            /// - On success, responds with [`DescribeTimeBasedAutoScalingOutput`](crate::output::DescribeTimeBasedAutoScalingOutput) with field(s):
    ///   - [`time_based_auto_scaling_configurations(Option<Vec<TimeBasedAutoScalingConfiguration>>)`](crate::output::DescribeTimeBasedAutoScalingOutput::time_based_auto_scaling_configurations): <p>An array of <code>TimeBasedAutoScalingConfiguration</code> objects that describe the configuration for the specified instances.</p>
                            /// - On failure, responds with [`SdkError<DescribeTimeBasedAutoScalingError>`](crate::error::DescribeTimeBasedAutoScalingError)
    pub fn describe_time_based_auto_scaling(&self) -> crate::client::fluent_builders::DescribeTimeBasedAutoScaling {
                                crate::client::fluent_builders::DescribeTimeBasedAutoScaling::new(self.handle.clone())
                            }
}

