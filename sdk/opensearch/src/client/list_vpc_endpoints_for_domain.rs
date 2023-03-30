// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListVpcEndpointsForDomain`](crate::client::fluent_builders::ListVpcEndpointsForDomain) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::ListVpcEndpointsForDomain::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::ListVpcEndpointsForDomain::set_domain_name): <p>The name of the domain to list associated VPC endpoints for.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListVpcEndpointsForDomain::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListVpcEndpointsForDomain::set_next_token): <p>If your initial <code>ListEndpointsForDomain</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListEndpointsForDomain</code> operations, which returns results in the next page.</p>
                            /// - On success, responds with [`ListVpcEndpointsForDomainOutput`](crate::output::ListVpcEndpointsForDomainOutput) with field(s):
    ///   - [`vpc_endpoint_summary_list(Option<Vec<VpcEndpointSummary>>)`](crate::output::ListVpcEndpointsForDomainOutput::vpc_endpoint_summary_list): <p>Information about each endpoint associated with the domain.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListVpcEndpointsForDomainOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
                            /// - On failure, responds with [`SdkError<ListVpcEndpointsForDomainError>`](crate::error::ListVpcEndpointsForDomainError)
    pub fn list_vpc_endpoints_for_domain(&self) -> crate::client::fluent_builders::ListVpcEndpointsForDomain {
                                crate::client::fluent_builders::ListVpcEndpointsForDomain::new(self.handle.clone())
                            }
}

