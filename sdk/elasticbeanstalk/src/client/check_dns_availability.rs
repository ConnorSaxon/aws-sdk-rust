// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CheckDNSAvailability`](crate::client::fluent_builders::CheckDNSAvailability) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cname_prefix(impl Into<String>)`](crate::client::fluent_builders::CheckDNSAvailability::cname_prefix) / [`set_cname_prefix(Option<String>)`](crate::client::fluent_builders::CheckDNSAvailability::set_cname_prefix): <p>The prefix used when this CNAME is reserved.</p>
                            /// - On success, responds with [`CheckDnsAvailabilityOutput`](crate::output::CheckDnsAvailabilityOutput) with field(s):
    ///   - [`available(Option<bool>)`](crate::output::CheckDnsAvailabilityOutput::available): <p>Indicates if the specified CNAME is available:</p>  <ul>   <li> <p> <code>true</code> : The CNAME is available.</p> </li>   <li> <p> <code>false</code> : The CNAME is not available.</p> </li>  </ul>
    ///   - [`fully_qualified_cname(Option<String>)`](crate::output::CheckDnsAvailabilityOutput::fully_qualified_cname): <p>The fully qualified CNAME to reserve when <code>CreateEnvironment</code> is called with the provided prefix.</p>
                            /// - On failure, responds with [`SdkError<CheckDNSAvailabilityError>`](crate::error::CheckDNSAvailabilityError)
    pub fn check_dns_availability(&self) -> crate::client::fluent_builders::CheckDNSAvailability {
                                crate::client::fluent_builders::CheckDNSAvailability::new(self.handle.clone())
                            }
}

