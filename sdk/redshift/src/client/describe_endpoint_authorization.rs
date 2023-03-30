// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeEndpointAuthorization`](crate::client::fluent_builders::DescribeEndpointAuthorization) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeEndpointAuthorization::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::DescribeEndpointAuthorization::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::client::fluent_builders::DescribeEndpointAuthorization::set_cluster_identifier): <p>The cluster identifier of the cluster to access.</p>
    ///   - [`account(impl Into<String>)`](crate::client::fluent_builders::DescribeEndpointAuthorization::account) / [`set_account(Option<String>)`](crate::client::fluent_builders::DescribeEndpointAuthorization::set_account): <p>The AAmazon Web Services account ID of either the cluster owner (grantor) or grantee. If <code>Grantee</code> parameter is true, then the <code>Account</code> value is of the grantor.</p>
    ///   - [`grantee(bool)`](crate::client::fluent_builders::DescribeEndpointAuthorization::grantee) / [`set_grantee(Option<bool>)`](crate::client::fluent_builders::DescribeEndpointAuthorization::set_grantee): <p>Indicates whether to check authorization from a grantor or grantee point of view. If true, Amazon Redshift returns endpoint authorizations that you've been granted. If false (default), checks authorization from a grantor point of view.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeEndpointAuthorization::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeEndpointAuthorization::set_max_records): <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a <code>Marker</code> is included in the response so that the remaining results can be retrieved.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeEndpointAuthorization::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeEndpointAuthorization::set_marker): <p>An optional pagination token provided by a previous <code>DescribeEndpointAuthorization</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by the <code>MaxRecords</code> parameter.</p>
                            /// - On success, responds with [`DescribeEndpointAuthorizationOutput`](crate::output::DescribeEndpointAuthorizationOutput) with field(s):
    ///   - [`endpoint_authorization_list(Option<Vec<EndpointAuthorization>>)`](crate::output::DescribeEndpointAuthorizationOutput::endpoint_authorization_list): <p>The authorizations to an endpoint.</p>
    ///   - [`marker(Option<String>)`](crate::output::DescribeEndpointAuthorizationOutput::marker): <p>An optional pagination token provided by a previous <code>DescribeEndpointAuthorization</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by the <code>MaxRecords</code> parameter.</p>
                            /// - On failure, responds with [`SdkError<DescribeEndpointAuthorizationError>`](crate::error::DescribeEndpointAuthorizationError)
    pub fn describe_endpoint_authorization(&self) -> crate::client::fluent_builders::DescribeEndpointAuthorization {
                                crate::client::fluent_builders::DescribeEndpointAuthorization::new(self.handle.clone())
                            }
}

