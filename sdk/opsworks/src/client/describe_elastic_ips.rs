// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeElasticIps`](crate::client::fluent_builders::DescribeElasticIps) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::DescribeElasticIps::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::DescribeElasticIps::set_instance_id): <p>The instance ID. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the Elastic IP addresses associated with the specified instance.</p>
    ///   - [`stack_id(impl Into<String>)`](crate::client::fluent_builders::DescribeElasticIps::stack_id) / [`set_stack_id(Option<String>)`](crate::client::fluent_builders::DescribeElasticIps::set_stack_id): <p>A stack ID. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the Elastic IP addresses that are registered with the specified stack.</p>
    ///   - [`ips(Vec<String>)`](crate::client::fluent_builders::DescribeElasticIps::ips) / [`set_ips(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeElasticIps::set_ips): <p>An array of Elastic IP addresses to be described. If you include this parameter, <code>DescribeElasticIps</code> returns a description of the specified Elastic IP addresses. Otherwise, it returns a description of every Elastic IP address.</p>
                            /// - On success, responds with [`DescribeElasticIpsOutput`](crate::output::DescribeElasticIpsOutput) with field(s):
    ///   - [`elastic_ips(Option<Vec<ElasticIp>>)`](crate::output::DescribeElasticIpsOutput::elastic_ips): <p>An <code>ElasticIps</code> object that describes the specified Elastic IP addresses.</p>
                            /// - On failure, responds with [`SdkError<DescribeElasticIpsError>`](crate::error::DescribeElasticIpsError)
    pub fn describe_elastic_ips(&self) -> crate::client::fluent_builders::DescribeElasticIps {
                                crate::client::fluent_builders::DescribeElasticIps::new(self.handle.clone())
                            }
}

