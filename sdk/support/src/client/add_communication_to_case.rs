// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AddCommunicationToCase`](crate::client::fluent_builders::AddCommunicationToCase) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`case_id(impl Into<String>)`](crate::client::fluent_builders::AddCommunicationToCase::case_id) / [`set_case_id(Option<String>)`](crate::client::fluent_builders::AddCommunicationToCase::set_case_id): <p>The support case ID requested or returned in the call. The case ID is an alphanumeric string formatted as shown in this example: case-<i>12345678910-2013-c4c1d2bf33c5cf47</i> </p>
    ///   - [`communication_body(impl Into<String>)`](crate::client::fluent_builders::AddCommunicationToCase::communication_body) / [`set_communication_body(Option<String>)`](crate::client::fluent_builders::AddCommunicationToCase::set_communication_body): <p>The body of an email communication to add to the support case.</p>
    ///   - [`cc_email_addresses(Vec<String>)`](crate::client::fluent_builders::AddCommunicationToCase::cc_email_addresses) / [`set_cc_email_addresses(Option<Vec<String>>)`](crate::client::fluent_builders::AddCommunicationToCase::set_cc_email_addresses): <p>The email addresses in the CC line of an email to be added to the support case.</p>
    ///   - [`attachment_set_id(impl Into<String>)`](crate::client::fluent_builders::AddCommunicationToCase::attachment_set_id) / [`set_attachment_set_id(Option<String>)`](crate::client::fluent_builders::AddCommunicationToCase::set_attachment_set_id): <p>The ID of a set of one or more attachments for the communication to add to the case. Create the set by calling <code>AddAttachmentsToSet</code> </p>
                            /// - On success, responds with [`AddCommunicationToCaseOutput`](crate::output::AddCommunicationToCaseOutput) with field(s):
    ///   - [`result(bool)`](crate::output::AddCommunicationToCaseOutput::result): <p>True if <code>AddCommunicationToCase</code> succeeds. Otherwise, returns an error.</p>
                            /// - On failure, responds with [`SdkError<AddCommunicationToCaseError>`](crate::error::AddCommunicationToCaseError)
    pub fn add_communication_to_case(&self) -> crate::client::fluent_builders::AddCommunicationToCase {
                                crate::client::fluent_builders::AddCommunicationToCase::new(self.handle.clone())
                            }
}

