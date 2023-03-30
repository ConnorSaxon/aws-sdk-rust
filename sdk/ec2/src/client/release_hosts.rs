// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ReleaseHosts`](crate::client::fluent_builders::ReleaseHosts) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`host_ids(Vec<String>)`](crate::client::fluent_builders::ReleaseHosts::host_ids) / [`set_host_ids(Option<Vec<String>>)`](crate::client::fluent_builders::ReleaseHosts::set_host_ids): <p>The IDs of the Dedicated Hosts to release.</p>
                            /// - On success, responds with [`ReleaseHostsOutput`](crate::output::ReleaseHostsOutput) with field(s):
    ///   - [`successful(Option<Vec<String>>)`](crate::output::ReleaseHostsOutput::successful): <p>The IDs of the Dedicated Hosts that were successfully released.</p>
    ///   - [`unsuccessful(Option<Vec<UnsuccessfulItem>>)`](crate::output::ReleaseHostsOutput::unsuccessful): <p>The IDs of the Dedicated Hosts that could not be released, including an error message.</p>
                            /// - On failure, responds with [`SdkError<ReleaseHostsError>`](crate::error::ReleaseHostsError)
    pub fn release_hosts(&self) -> crate::client::fluent_builders::ReleaseHosts {
                                crate::client::fluent_builders::ReleaseHosts::new(self.handle.clone())
                            }
}

