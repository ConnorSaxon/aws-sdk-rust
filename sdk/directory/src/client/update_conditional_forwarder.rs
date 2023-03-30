// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateConditionalForwarder`](crate::client::fluent_builders::UpdateConditionalForwarder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::client::fluent_builders::UpdateConditionalForwarder::directory_id) / [`set_directory_id(Option<String>)`](crate::client::fluent_builders::UpdateConditionalForwarder::set_directory_id): <p>The directory ID of the Amazon Web Services directory for which to update the conditional forwarder.</p>
    ///   - [`remote_domain_name(impl Into<String>)`](crate::client::fluent_builders::UpdateConditionalForwarder::remote_domain_name) / [`set_remote_domain_name(Option<String>)`](crate::client::fluent_builders::UpdateConditionalForwarder::set_remote_domain_name): <p>The fully qualified domain name (FQDN) of the remote domain with which you will set up a trust relationship.</p>
    ///   - [`dns_ip_addrs(Vec<String>)`](crate::client::fluent_builders::UpdateConditionalForwarder::dns_ip_addrs) / [`set_dns_ip_addrs(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateConditionalForwarder::set_dns_ip_addrs): <p>The updated IP addresses of the remote DNS server associated with the conditional forwarder.</p>
                            /// - On success, responds with [`UpdateConditionalForwarderOutput`](crate::output::UpdateConditionalForwarderOutput)
                            /// - On failure, responds with [`SdkError<UpdateConditionalForwarderError>`](crate::error::UpdateConditionalForwarderError)
    pub fn update_conditional_forwarder(&self) -> crate::client::fluent_builders::UpdateConditionalForwarder {
                                crate::client::fluent_builders::UpdateConditionalForwarder::new(self.handle.clone())
                            }
}

