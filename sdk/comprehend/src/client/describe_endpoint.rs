// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeEndpoint`](crate::client::fluent_builders::DescribeEndpoint) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`endpoint_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeEndpoint::endpoint_arn) / [`set_endpoint_arn(Option<String>)`](crate::client::fluent_builders::DescribeEndpoint::set_endpoint_arn): <p>The Amazon Resource Number (ARN) of the endpoint being described.</p>
                            /// - On success, responds with [`DescribeEndpointOutput`](crate::output::DescribeEndpointOutput) with field(s):
    ///   - [`endpoint_properties(Option<EndpointProperties>)`](crate::output::DescribeEndpointOutput::endpoint_properties): <p>Describes information associated with the specific endpoint.</p>
                            /// - On failure, responds with [`SdkError<DescribeEndpointError>`](crate::error::DescribeEndpointError)
    pub fn describe_endpoint(&self) -> crate::client::fluent_builders::DescribeEndpoint {
                                crate::client::fluent_builders::DescribeEndpoint::new(self.handle.clone())
                            }
}

