// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAuditStreamConfiguration`](crate::client::fluent_builders::DescribeAuditStreamConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeAuditStreamConfiguration::fleet_arn) / [`set_fleet_arn(Option<String>)`](crate::client::fluent_builders::DescribeAuditStreamConfiguration::set_fleet_arn): <p>The ARN of the fleet.</p>
                            /// - On success, responds with [`DescribeAuditStreamConfigurationOutput`](crate::output::DescribeAuditStreamConfigurationOutput) with field(s):
    ///   - [`audit_stream_arn(Option<String>)`](crate::output::DescribeAuditStreamConfigurationOutput::audit_stream_arn): <p>The ARN of the Amazon Kinesis data stream that will receive the audit events.</p>
                            /// - On failure, responds with [`SdkError<DescribeAuditStreamConfigurationError>`](crate::error::DescribeAuditStreamConfigurationError)
    #[deprecated(note = "Amazon WorkLink is no longer supported. This will be removed in a future version of the SDK.")]
    pub fn describe_audit_stream_configuration(&self) -> crate::client::fluent_builders::DescribeAuditStreamConfiguration {
                                crate::client::fluent_builders::DescribeAuditStreamConfiguration::new(self.handle.clone())
                            }
}

