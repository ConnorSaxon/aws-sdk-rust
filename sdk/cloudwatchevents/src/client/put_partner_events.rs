// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutPartnerEvents`](crate::client::fluent_builders::PutPartnerEvents) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`entries(Vec<PutPartnerEventsRequestEntry>)`](crate::client::fluent_builders::PutPartnerEvents::entries) / [`set_entries(Option<Vec<PutPartnerEventsRequestEntry>>)`](crate::client::fluent_builders::PutPartnerEvents::set_entries): <p>The list of events to write to the event bus.</p>
                            /// - On success, responds with [`PutPartnerEventsOutput`](crate::output::PutPartnerEventsOutput) with field(s):
    ///   - [`failed_entry_count(i32)`](crate::output::PutPartnerEventsOutput::failed_entry_count): <p>The number of events from this operation that could not be written to the partner event bus.</p>
    ///   - [`entries(Option<Vec<PutPartnerEventsResultEntry>>)`](crate::output::PutPartnerEventsOutput::entries): <p>The list of events from this operation that were successfully written to the partner event bus.</p>
                            /// - On failure, responds with [`SdkError<PutPartnerEventsError>`](crate::error::PutPartnerEventsError)
    pub fn put_partner_events(&self) -> crate::client::fluent_builders::PutPartnerEvents {
                                crate::client::fluent_builders::PutPartnerEvents::new(self.handle.clone())
                            }
}

