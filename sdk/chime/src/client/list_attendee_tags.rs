// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAttendeeTags`](crate::client::fluent_builders::ListAttendeeTags) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`meeting_id(impl Into<String>)`](crate::client::fluent_builders::ListAttendeeTags::meeting_id) / [`set_meeting_id(Option<String>)`](crate::client::fluent_builders::ListAttendeeTags::set_meeting_id): <p>The Amazon Chime SDK meeting ID.</p>
    ///   - [`attendee_id(impl Into<String>)`](crate::client::fluent_builders::ListAttendeeTags::attendee_id) / [`set_attendee_id(Option<String>)`](crate::client::fluent_builders::ListAttendeeTags::set_attendee_id): <p>The Amazon Chime SDK attendee ID.</p>
                            /// - On success, responds with [`ListAttendeeTagsOutput`](crate::output::ListAttendeeTagsOutput) with field(s):
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::ListAttendeeTagsOutput::tags): <p>A list of tag key-value pairs.</p>
                            /// - On failure, responds with [`SdkError<ListAttendeeTagsError>`](crate::error::ListAttendeeTagsError)
    pub fn list_attendee_tags(&self) -> crate::client::fluent_builders::ListAttendeeTags {
                                crate::client::fluent_builders::ListAttendeeTags::new(self.handle.clone())
                            }
}

