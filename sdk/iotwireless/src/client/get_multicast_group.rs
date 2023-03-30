// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetMulticastGroup`](crate::client::fluent_builders::GetMulticastGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetMulticastGroup::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetMulticastGroup::set_id): <p>The ID of the multicast group.</p>
                            /// - On success, responds with [`GetMulticastGroupOutput`](crate::output::GetMulticastGroupOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::GetMulticastGroupOutput::arn): <p>The arn of the multicast group.</p>
    ///   - [`id(Option<String>)`](crate::output::GetMulticastGroupOutput::id): <p>The ID of the multicast group.</p>
    ///   - [`name(Option<String>)`](crate::output::GetMulticastGroupOutput::name): <p>The name of the multicast group.</p>
    ///   - [`description(Option<String>)`](crate::output::GetMulticastGroupOutput::description): <p>The description of the new resource.</p>
    ///   - [`status(Option<String>)`](crate::output::GetMulticastGroupOutput::status): <p>The status of the multicast group.</p>
    ///   - [`lo_ra_wan(Option<LoRaWanMulticastGet>)`](crate::output::GetMulticastGroupOutput::lo_ra_wan): <p>The LoRaWAN information that is to be returned from getting multicast group information.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::output::GetMulticastGroupOutput::created_at): <p>Created at timestamp for the resource.</p>
                            /// - On failure, responds with [`SdkError<GetMulticastGroupError>`](crate::error::GetMulticastGroupError)
    pub fn get_multicast_group(&self) -> crate::client::fluent_builders::GetMulticastGroup {
                                crate::client::fluent_builders::GetMulticastGroup::new(self.handle.clone())
                            }
}

