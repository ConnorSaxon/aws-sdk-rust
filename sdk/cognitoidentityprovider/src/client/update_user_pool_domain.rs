// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateUserPoolDomain`](crate::client::fluent_builders::UpdateUserPoolDomain) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::UpdateUserPoolDomain::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::UpdateUserPoolDomain::set_domain): <p>The domain name for the custom domain that hosts the sign-up and sign-in pages for your application. One example might be <code>auth.example.com</code>. </p>  <p>This string can include only lowercase letters, numbers, and hyphens. Don't use a hyphen for the first or last character. Use periods to separate subdomain names.</p>
    ///   - [`user_pool_id(impl Into<String>)`](crate::client::fluent_builders::UpdateUserPoolDomain::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::client::fluent_builders::UpdateUserPoolDomain::set_user_pool_id): <p>The ID of the user pool that is associated with the custom domain whose certificate you're updating.</p>
    ///   - [`custom_domain_config(CustomDomainConfigType)`](crate::client::fluent_builders::UpdateUserPoolDomain::custom_domain_config) / [`set_custom_domain_config(Option<CustomDomainConfigType>)`](crate::client::fluent_builders::UpdateUserPoolDomain::set_custom_domain_config): <p>The configuration for a custom domain that hosts the sign-up and sign-in pages for your application. Use this object to specify an SSL certificate that is managed by ACM.</p>
                            /// - On success, responds with [`UpdateUserPoolDomainOutput`](crate::output::UpdateUserPoolDomainOutput) with field(s):
    ///   - [`cloud_front_domain(Option<String>)`](crate::output::UpdateUserPoolDomainOutput::cloud_front_domain): <p>The Amazon CloudFront endpoint that Amazon Cognito set up when you added the custom domain to your user pool.</p>
                            /// - On failure, responds with [`SdkError<UpdateUserPoolDomainError>`](crate::error::UpdateUserPoolDomainError)
    pub fn update_user_pool_domain(&self) -> crate::client::fluent_builders::UpdateUserPoolDomain {
                                crate::client::fluent_builders::UpdateUserPoolDomain::new(self.handle.clone())
                            }
}

