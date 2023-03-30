// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPermissions`](crate::client::fluent_builders::ListPermissions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPermissions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`certificate_authority_arn(impl Into<String>)`](crate::client::fluent_builders::ListPermissions::certificate_authority_arn) / [`set_certificate_authority_arn(Option<String>)`](crate::client::fluent_builders::ListPermissions::set_certificate_authority_arn): <p>The Amazon Resource Number (ARN) of the private CA to inspect. You can find the ARN by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_ListCertificateAuthorities.html">ListCertificateAuthorities</a> action. This must be of the form: <code>arn:aws:acm-pca:region:account:certificate-authority/12345678-1234-1234-1234-123456789012</code> You can get a private CA's ARN by running the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_ListCertificateAuthorities.html">ListCertificateAuthorities</a> action.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPermissions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPermissions::set_next_token): <p>When paginating results, use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <b>NextToken</b> from the response you just received.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPermissions::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListPermissions::set_max_results): <p>When paginating results, use this parameter to specify the maximum number of items to return in the response. If additional items exist beyond the number you specify, the <b>NextToken</b> element is sent in the response. Use this <b>NextToken</b> value in a subsequent request to retrieve additional items.</p>
                            /// - On success, responds with [`ListPermissionsOutput`](crate::output::ListPermissionsOutput) with field(s):
    ///   - [`permissions(Option<Vec<Permission>>)`](crate::output::ListPermissionsOutput::permissions): <p>Summary information about each permission assigned by the specified private CA, including the action enabled, the policy provided, and the time of creation.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPermissionsOutput::next_token): <p>When the list is truncated, this value is present and should be used for the <b>NextToken</b> parameter in a subsequent pagination request. </p>
                            /// - On failure, responds with [`SdkError<ListPermissionsError>`](crate::error::ListPermissionsError)
    pub fn list_permissions(&self) -> crate::client::fluent_builders::ListPermissions {
                                crate::client::fluent_builders::ListPermissions::new(self.handle.clone())
                            }
}

