// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetMulticastGroupSession`](crate::client::fluent_builders::GetMulticastGroupSession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetMulticastGroupSession::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetMulticastGroupSession::set_id): <p>The ID of the multicast group.</p>
                            /// - On success, responds with [`GetMulticastGroupSessionOutput`](crate::output::GetMulticastGroupSessionOutput) with field(s):
    ///   - [`lo_ra_wan(Option<LoRaWanMulticastSession>)`](crate::output::GetMulticastGroupSessionOutput::lo_ra_wan): <p>The LoRaWAN information used with the multicast session.</p>
                            /// - On failure, responds with [`SdkError<GetMulticastGroupSessionError>`](crate::error::GetMulticastGroupSessionError)
    pub fn get_multicast_group_session(&self) -> crate::client::fluent_builders::GetMulticastGroupSession {
                                crate::client::fluent_builders::GetMulticastGroupSession::new(self.handle.clone())
                            }
}

