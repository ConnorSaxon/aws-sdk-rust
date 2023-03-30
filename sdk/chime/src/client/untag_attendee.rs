// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UntagAttendee`](crate::client::fluent_builders::UntagAttendee) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`meeting_id(impl Into<String>)`](crate::client::fluent_builders::UntagAttendee::meeting_id) / [`set_meeting_id(Option<String>)`](crate::client::fluent_builders::UntagAttendee::set_meeting_id): <p>The Amazon Chime SDK meeting ID.</p>
    ///   - [`attendee_id(impl Into<String>)`](crate::client::fluent_builders::UntagAttendee::attendee_id) / [`set_attendee_id(Option<String>)`](crate::client::fluent_builders::UntagAttendee::set_attendee_id): <p>The Amazon Chime SDK attendee ID.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::client::fluent_builders::UntagAttendee::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UntagAttendee::set_tag_keys): <p>The tag keys.</p>
                            /// - On success, responds with [`UntagAttendeeOutput`](crate::output::UntagAttendeeOutput)
                            /// - On failure, responds with [`SdkError<UntagAttendeeError>`](crate::error::UntagAttendeeError)
    pub fn untag_attendee(&self) -> crate::client::fluent_builders::UntagAttendee {
                                crate::client::fluent_builders::UntagAttendee::new(self.handle.clone())
                            }
}

