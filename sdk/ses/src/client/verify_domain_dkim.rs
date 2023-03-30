// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`VerifyDomainDkim`](crate::client::fluent_builders::VerifyDomainDkim) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::client::fluent_builders::VerifyDomainDkim::domain) / [`set_domain(Option<String>)`](crate::client::fluent_builders::VerifyDomainDkim::set_domain): <p>The name of the domain to be verified for Easy DKIM signing.</p>
                            /// - On success, responds with [`VerifyDomainDkimOutput`](crate::output::VerifyDomainDkimOutput) with field(s):
    ///   - [`dkim_tokens(Option<Vec<String>>)`](crate::output::VerifyDomainDkimOutput::dkim_tokens): <p>A set of character strings that represent the domain's identity. If the identity is an email address, the tokens represent the domain of that address.</p>  <p>Using these tokens, you need to create DNS CNAME records that point to DKIM public keys that are hosted by Amazon SES. Amazon Web Services eventually detects that you've updated your DNS records. This detection process might take up to 72 hours. After successful detection, Amazon SES is able to DKIM-sign email originating from that domain. (This only applies to domain identities, not email address identities.)</p>  <p>For more information about creating DNS records using DKIM tokens, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Amazon SES Developer Guide</a>.</p>
                            /// - On failure, responds with [`SdkError<VerifyDomainDkimError>`](crate::error::VerifyDomainDkimError)
    pub fn verify_domain_dkim(&self) -> crate::client::fluent_builders::VerifyDomainDkim {
                                crate::client::fluent_builders::VerifyDomainDkim::new(self.handle.clone())
                            }
}

