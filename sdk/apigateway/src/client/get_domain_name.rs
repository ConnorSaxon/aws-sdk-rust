// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDomainName`](crate::client::fluent_builders::GetDomainName) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::GetDomainName::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::GetDomainName::set_domain_name): <p>The name of the DomainName resource.</p>
                            /// - On success, responds with [`GetDomainNameOutput`](crate::output::GetDomainNameOutput) with field(s):
    ///   - [`domain_name(Option<String>)`](crate::output::GetDomainNameOutput::domain_name): <p>The custom domain name as an API host name, for example, <code>my-api.example.com</code>.</p>
    ///   - [`certificate_name(Option<String>)`](crate::output::GetDomainNameOutput::certificate_name): <p>The name of the certificate that will be used by edge-optimized endpoint for this domain name.</p>
    ///   - [`certificate_arn(Option<String>)`](crate::output::GetDomainNameOutput::certificate_arn): <p>The reference to an AWS-managed certificate that will be used by edge-optimized endpoint for this domain name. AWS Certificate Manager is the only supported source.</p>
    ///   - [`certificate_upload_date(Option<DateTime>)`](crate::output::GetDomainNameOutput::certificate_upload_date): <p>The timestamp when the certificate that was used by edge-optimized endpoint for this domain name was uploaded.</p>
    ///   - [`regional_domain_name(Option<String>)`](crate::output::GetDomainNameOutput::regional_domain_name): <p>The domain name associated with the regional endpoint for this custom domain name. You set up this association by adding a DNS record that points the custom domain name to this regional domain name. The regional domain name is returned by API Gateway when you create a regional endpoint.</p>
    ///   - [`regional_hosted_zone_id(Option<String>)`](crate::output::GetDomainNameOutput::regional_hosted_zone_id): <p>The region-specific Amazon Route 53 Hosted Zone ID of the regional endpoint. For more information, see Set up a Regional Custom Domain Name and AWS Regions and Endpoints for API Gateway. </p>
    ///   - [`regional_certificate_name(Option<String>)`](crate::output::GetDomainNameOutput::regional_certificate_name): <p>The name of the certificate that will be used for validating the regional domain name.</p>
    ///   - [`regional_certificate_arn(Option<String>)`](crate::output::GetDomainNameOutput::regional_certificate_arn): <p>The reference to an AWS-managed certificate that will be used for validating the regional domain name. AWS Certificate Manager is the only supported source.</p>
    ///   - [`distribution_domain_name(Option<String>)`](crate::output::GetDomainNameOutput::distribution_domain_name): <p>The domain name of the Amazon CloudFront distribution associated with this custom domain name for an edge-optimized endpoint. You set up this association when adding a DNS record pointing the custom domain name to this distribution name. For more information about CloudFront distributions, see the Amazon CloudFront documentation.</p>
    ///   - [`distribution_hosted_zone_id(Option<String>)`](crate::output::GetDomainNameOutput::distribution_hosted_zone_id): <p>The region-agnostic Amazon Route 53 Hosted Zone ID of the edge-optimized endpoint. The valid value is <code>Z2FDTNDATAQYW2</code> for all the regions. For more information, see Set up a Regional Custom Domain Name and AWS Regions and Endpoints for API Gateway. </p>
    ///   - [`endpoint_configuration(Option<EndpointConfiguration>)`](crate::output::GetDomainNameOutput::endpoint_configuration): <p>The endpoint configuration of this DomainName showing the endpoint types of the domain name. </p>
    ///   - [`domain_name_status(Option<DomainNameStatus>)`](crate::output::GetDomainNameOutput::domain_name_status): <p>The status of the DomainName migration. The valid values are <code>AVAILABLE</code> and <code>UPDATING</code>. If the status is <code>UPDATING</code>, the domain cannot be modified further until the existing operation is complete. If it is <code>AVAILABLE</code>, the domain can be updated.</p>
    ///   - [`domain_name_status_message(Option<String>)`](crate::output::GetDomainNameOutput::domain_name_status_message): <p>An optional text message containing detailed information about status of the DomainName migration.</p>
    ///   - [`security_policy(Option<SecurityPolicy>)`](crate::output::GetDomainNameOutput::security_policy): <p>The Transport Layer Security (TLS) version + cipher suite for this DomainName. The valid values are <code>TLS_1_0</code> and <code>TLS_1_2</code>.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::GetDomainNameOutput::tags): <p>The collection of tags. Each tag element is associated with a given resource.</p>
    ///   - [`mutual_tls_authentication(Option<MutualTlsAuthentication>)`](crate::output::GetDomainNameOutput::mutual_tls_authentication): <p>The mutual TLS authentication configuration for a custom domain name. If specified, API Gateway performs two-way authentication between the client and the server. Clients must present a trusted certificate to access your API.</p>
    ///   - [`ownership_verification_certificate_arn(Option<String>)`](crate::output::GetDomainNameOutput::ownership_verification_certificate_arn): <p>The ARN of the public certificate issued by ACM to validate ownership of your custom domain. Only required when configuring mutual TLS and using an ACM imported or private CA certificate ARN as the regionalCertificateArn.</p>
                            /// - On failure, responds with [`SdkError<GetDomainNameError>`](crate::error::GetDomainNameError)
    pub fn get_domain_name(&self) -> crate::client::fluent_builders::GetDomainName {
                                crate::client::fluent_builders::GetDomainName::new(self.handle.clone())
                            }
}

