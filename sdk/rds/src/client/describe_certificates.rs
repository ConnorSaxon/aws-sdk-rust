// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeCertificates`](crate::client::fluent_builders::DescribeCertificates) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeCertificates::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`certificate_identifier(impl Into<String>)`](crate::client::fluent_builders::DescribeCertificates::certificate_identifier) / [`set_certificate_identifier(Option<String>)`](crate::client::fluent_builders::DescribeCertificates::set_certificate_identifier): <p>The user-supplied certificate identifier. If this parameter is specified, information for only the identified certificate is returned. This parameter isn't case-sensitive.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must match an existing CertificateIdentifier.</p> </li>  </ul>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeCertificates::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeCertificates::set_filters): <p>This parameter isn't currently supported.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::DescribeCertificates::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::DescribeCertificates::set_max_records): <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so you can retrieve the remaining results.</p>  <p>Default: 100</p>  <p>Constraints: Minimum 20, maximum 100.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeCertificates::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeCertificates::set_marker): <p>An optional pagination token provided by a previous <code>DescribeCertificates</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
                            /// - On success, responds with [`DescribeCertificatesOutput`](crate::output::DescribeCertificatesOutput) with field(s):
    ///   - [`certificates(Option<Vec<Certificate>>)`](crate::output::DescribeCertificatesOutput::certificates): <p>The list of <code>Certificate</code> objects for the Amazon Web Services account.</p>
    ///   - [`marker(Option<String>)`](crate::output::DescribeCertificatesOutput::marker): <p>An optional pagination token provided by a previous <code>DescribeCertificates</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code> .</p>
                            /// - On failure, responds with [`SdkError<DescribeCertificatesError>`](crate::error::DescribeCertificatesError)
    pub fn describe_certificates(&self) -> crate::client::fluent_builders::DescribeCertificates {
                                crate::client::fluent_builders::DescribeCertificates::new(self.handle.clone())
                            }
}

