// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeElasticsearchInstanceTypeLimits`](crate::client::fluent_builders::DescribeElasticsearchInstanceTypeLimits) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::DescribeElasticsearchInstanceTypeLimits::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::DescribeElasticsearchInstanceTypeLimits::set_domain_name): <p> DomainName represents the name of the Domain that we are trying to modify. This should be present only if we are querying for Elasticsearch <code> <code>Limits</code> </code> for existing domain. </p>
    ///   - [`instance_type(EsPartitionInstanceType)`](crate::client::fluent_builders::DescribeElasticsearchInstanceTypeLimits::instance_type) / [`set_instance_type(Option<EsPartitionInstanceType>)`](crate::client::fluent_builders::DescribeElasticsearchInstanceTypeLimits::set_instance_type): <p> The instance type for an Elasticsearch cluster for which Elasticsearch <code> <code>Limits</code> </code> are needed. </p>
    ///   - [`elasticsearch_version(impl Into<String>)`](crate::client::fluent_builders::DescribeElasticsearchInstanceTypeLimits::elasticsearch_version) / [`set_elasticsearch_version(Option<String>)`](crate::client::fluent_builders::DescribeElasticsearchInstanceTypeLimits::set_elasticsearch_version): <p> Version of Elasticsearch for which <code> <code>Limits</code> </code> are needed. </p>
                            /// - On success, responds with [`DescribeElasticsearchInstanceTypeLimitsOutput`](crate::output::DescribeElasticsearchInstanceTypeLimitsOutput) with field(s):
    ///   - [`limits_by_role(Option<HashMap<String, Limits>>)`](crate::output::DescribeElasticsearchInstanceTypeLimitsOutput::limits_by_role): <p> Map of Role of the Instance and Limits that are applicable. Role performed by given Instance in Elasticsearch can be one of the following: </p> <ul>   <li>data: If the given InstanceType is used as data node</li>   <li>master: If the given InstanceType is used as master node</li>   <li>ultra_warm: If the given InstanceType is used as warm node</li>  </ul>  <p></p>
                            /// - On failure, responds with [`SdkError<DescribeElasticsearchInstanceTypeLimitsError>`](crate::error::DescribeElasticsearchInstanceTypeLimitsError)
    pub fn describe_elasticsearch_instance_type_limits(&self) -> crate::client::fluent_builders::DescribeElasticsearchInstanceTypeLimits {
                                crate::client::fluent_builders::DescribeElasticsearchInstanceTypeLimits::new(self.handle.clone())
                            }
}

