// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeEnvironmentHealth`](crate::client::fluent_builders::DescribeEnvironmentHealth) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`environment_name(impl Into<String>)`](crate::client::fluent_builders::DescribeEnvironmentHealth::environment_name) / [`set_environment_name(Option<String>)`](crate::client::fluent_builders::DescribeEnvironmentHealth::set_environment_name): <p>Specify the environment by name.</p>  <p>You must specify either this or an EnvironmentName, or both.</p>
    ///   - [`environment_id(impl Into<String>)`](crate::client::fluent_builders::DescribeEnvironmentHealth::environment_id) / [`set_environment_id(Option<String>)`](crate::client::fluent_builders::DescribeEnvironmentHealth::set_environment_id): <p>Specify the environment by ID.</p>  <p>You must specify either this or an EnvironmentName, or both.</p>
    ///   - [`attribute_names(Vec<EnvironmentHealthAttribute>)`](crate::client::fluent_builders::DescribeEnvironmentHealth::attribute_names) / [`set_attribute_names(Option<Vec<EnvironmentHealthAttribute>>)`](crate::client::fluent_builders::DescribeEnvironmentHealth::set_attribute_names): <p>Specify the response elements to return. To retrieve all attributes, set to <code>All</code>. If no attribute names are specified, returns the name of the environment.</p>
                            /// - On success, responds with [`DescribeEnvironmentHealthOutput`](crate::output::DescribeEnvironmentHealthOutput) with field(s):
    ///   - [`environment_name(Option<String>)`](crate::output::DescribeEnvironmentHealthOutput::environment_name): <p>The environment's name.</p>
    ///   - [`health_status(Option<String>)`](crate::output::DescribeEnvironmentHealthOutput::health_status): <p>The <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-status.html">health status</a> of the environment. For example, <code>Ok</code>.</p>
    ///   - [`status(Option<EnvironmentHealth>)`](crate::output::DescribeEnvironmentHealthOutput::status): <p>The environment's operational status. <code>Ready</code>, <code>Launching</code>, <code>Updating</code>, <code>Terminating</code>, or <code>Terminated</code>.</p>
    ///   - [`color(Option<String>)`](crate::output::DescribeEnvironmentHealthOutput::color): <p>The <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-status.html">health color</a> of the environment.</p>
    ///   - [`causes(Option<Vec<String>>)`](crate::output::DescribeEnvironmentHealthOutput::causes): <p>Descriptions of the data that contributed to the environment's current health status.</p>
    ///   - [`application_metrics(Option<ApplicationMetrics>)`](crate::output::DescribeEnvironmentHealthOutput::application_metrics): <p>Application request metrics for the environment.</p>
    ///   - [`instances_health(Option<InstanceHealthSummary>)`](crate::output::DescribeEnvironmentHealthOutput::instances_health): <p>Summary health information for the instances in the environment.</p>
    ///   - [`refreshed_at(Option<DateTime>)`](crate::output::DescribeEnvironmentHealthOutput::refreshed_at): <p>The date and time that the health information was retrieved.</p>
                            /// - On failure, responds with [`SdkError<DescribeEnvironmentHealthError>`](crate::error::DescribeEnvironmentHealthError)
    pub fn describe_environment_health(&self) -> crate::client::fluent_builders::DescribeEnvironmentHealth {
                                crate::client::fluent_builders::DescribeEnvironmentHealth::new(self.handle.clone())
                            }
}

