// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeConfiguration`](crate::client::fluent_builders::DescribeConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::DescribeConfiguration::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::DescribeConfiguration::set_arn): <p>The Amazon Resource Name (ARN) that uniquely identifies an MSK configuration and all of its revisions.</p>
                            /// - On success, responds with [`DescribeConfigurationOutput`](crate::output::DescribeConfigurationOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::DescribeConfigurationOutput::arn): <p>The Amazon Resource Name (ARN) of the configuration.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeConfigurationOutput::creation_time): <p>The time when the configuration was created.</p>
    ///   - [`description(Option<String>)`](crate::output::DescribeConfigurationOutput::description): <p>The description of the configuration.</p>
    ///   - [`kafka_versions(Option<Vec<String>>)`](crate::output::DescribeConfigurationOutput::kafka_versions): <p>The versions of Apache Kafka with which you can use this MSK configuration.</p>
    ///   - [`latest_revision(Option<ConfigurationRevision>)`](crate::output::DescribeConfigurationOutput::latest_revision): <p>Latest revision of the configuration.</p>
    ///   - [`name(Option<String>)`](crate::output::DescribeConfigurationOutput::name): <p>The name of the configuration.</p>
    ///   - [`state(Option<ConfigurationState>)`](crate::output::DescribeConfigurationOutput::state): <p>The state of the configuration. The possible states are ACTIVE, DELETING, and DELETE_FAILED. </p>
                            /// - On failure, responds with [`SdkError<DescribeConfigurationError>`](crate::error::DescribeConfigurationError)
    pub fn describe_configuration(&self) -> crate::client::fluent_builders::DescribeConfiguration {
                                crate::client::fluent_builders::DescribeConfiguration::new(self.handle.clone())
                            }
}

