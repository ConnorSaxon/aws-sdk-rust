// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeInstanceTypeLimits`](crate::client::fluent_builders::DescribeInstanceTypeLimits) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::DescribeInstanceTypeLimits::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::DescribeInstanceTypeLimits::set_domain_name): <p>The name of the domain. Only specify if you need the limits for an existing domain.</p>
    ///   - [`instance_type(OpenSearchPartitionInstanceType)`](crate::client::fluent_builders::DescribeInstanceTypeLimits::instance_type) / [`set_instance_type(Option<OpenSearchPartitionInstanceType>)`](crate::client::fluent_builders::DescribeInstanceTypeLimits::set_instance_type): <p>The OpenSearch Service instance type for which you need limit information.</p>
    ///   - [`engine_version(impl Into<String>)`](crate::client::fluent_builders::DescribeInstanceTypeLimits::engine_version) / [`set_engine_version(Option<String>)`](crate::client::fluent_builders::DescribeInstanceTypeLimits::set_engine_version): <p>Version of OpenSearch or Elasticsearch, in the format Elasticsearch_X.Y or OpenSearch_X.Y. Defaults to the latest version of OpenSearch.</p>
                            /// - On success, responds with [`DescribeInstanceTypeLimitsOutput`](crate::output::DescribeInstanceTypeLimitsOutput) with field(s):
    ///   - [`limits_by_role(Option<HashMap<String, Limits>>)`](crate::output::DescribeInstanceTypeLimitsOutput::limits_by_role): <p>Map that contains all applicable instance type limits.<code>data</code> refers to data nodes.<code>master</code> refers to dedicated master nodes.</p>
                            /// - On failure, responds with [`SdkError<DescribeInstanceTypeLimitsError>`](crate::error::DescribeInstanceTypeLimitsError)
    pub fn describe_instance_type_limits(&self) -> crate::client::fluent_builders::DescribeInstanceTypeLimits {
                                crate::client::fluent_builders::DescribeInstanceTypeLimits::new(self.handle.clone())
                            }
}

