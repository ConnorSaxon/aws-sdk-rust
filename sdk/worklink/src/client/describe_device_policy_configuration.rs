// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDevicePolicyConfiguration`](crate::client::fluent_builders::DescribeDevicePolicyConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeDevicePolicyConfiguration::fleet_arn) / [`set_fleet_arn(Option<String>)`](crate::client::fluent_builders::DescribeDevicePolicyConfiguration::set_fleet_arn): <p>The ARN of the fleet.</p>
                            /// - On success, responds with [`DescribeDevicePolicyConfigurationOutput`](crate::output::DescribeDevicePolicyConfigurationOutput) with field(s):
    ///   - [`device_ca_certificate(Option<String>)`](crate::output::DescribeDevicePolicyConfigurationOutput::device_ca_certificate): <p>The certificate chain, including intermediate certificates and the root certificate authority certificate used to issue device certificates.</p>
                            /// - On failure, responds with [`SdkError<DescribeDevicePolicyConfigurationError>`](crate::error::DescribeDevicePolicyConfigurationError)
    #[deprecated(note = "Amazon WorkLink is no longer supported. This will be removed in a future version of the SDK.")]
    pub fn describe_device_policy_configuration(&self) -> crate::client::fluent_builders::DescribeDevicePolicyConfiguration {
                                crate::client::fluent_builders::DescribeDevicePolicyConfiguration::new(self.handle.clone())
                            }
}

