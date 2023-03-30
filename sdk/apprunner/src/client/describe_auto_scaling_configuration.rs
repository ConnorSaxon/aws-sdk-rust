// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAutoScalingConfiguration`](crate::client::fluent_builders::DescribeAutoScalingConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`auto_scaling_configuration_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeAutoScalingConfiguration::auto_scaling_configuration_arn) / [`set_auto_scaling_configuration_arn(Option<String>)`](crate::client::fluent_builders::DescribeAutoScalingConfiguration::set_auto_scaling_configuration_arn): <p>The Amazon Resource Name (ARN) of the App Runner auto scaling configuration that you want a description for.</p>  <p>The ARN can be a full auto scaling configuration ARN, or a partial ARN ending with either <code>.../<i>name</i> </code> or <code>.../<i>name</i>/<i>revision</i> </code>. If a revision isn't specified, the latest active revision is described.</p>
                            /// - On success, responds with [`DescribeAutoScalingConfigurationOutput`](crate::output::DescribeAutoScalingConfigurationOutput) with field(s):
    ///   - [`auto_scaling_configuration(Option<AutoScalingConfiguration>)`](crate::output::DescribeAutoScalingConfigurationOutput::auto_scaling_configuration): <p>A full description of the App Runner auto scaling configuration that you specified in this request.</p>
                            /// - On failure, responds with [`SdkError<DescribeAutoScalingConfigurationError>`](crate::error::DescribeAutoScalingConfigurationError)
    pub fn describe_auto_scaling_configuration(&self) -> crate::client::fluent_builders::DescribeAutoScalingConfiguration {
                                crate::client::fluent_builders::DescribeAutoScalingConfiguration::new(self.handle.clone())
                            }
}

