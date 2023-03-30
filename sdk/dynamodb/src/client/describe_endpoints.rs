// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeEndpoints`](crate::client::fluent_builders::DescribeEndpoints) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DescribeEndpoints::send) it.
                            /// - On success, responds with [`DescribeEndpointsOutput`](crate::output::DescribeEndpointsOutput) with field(s):
    ///   - [`endpoints(Option<Vec<Endpoint>>)`](crate::output::DescribeEndpointsOutput::endpoints): <p>List of endpoints.</p>
                            /// - On failure, responds with [`SdkError<DescribeEndpointsError>`](crate::error::DescribeEndpointsError)
    pub fn describe_endpoints(&self) -> crate::client::fluent_builders::DescribeEndpoints {
                                crate::client::fluent_builders::DescribeEndpoints::new(self.handle.clone())
                            }
}

