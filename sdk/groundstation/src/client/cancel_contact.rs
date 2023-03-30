// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelContact`](crate::client::fluent_builders::CancelContact) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`contact_id(impl Into<String>)`](crate::client::fluent_builders::CancelContact::contact_id) / [`set_contact_id(Option<String>)`](crate::client::fluent_builders::CancelContact::set_contact_id): <p>UUID of a contact.</p>
                            /// - On success, responds with [`CancelContactOutput`](crate::output::CancelContactOutput) with field(s):
    ///   - [`contact_id(Option<String>)`](crate::output::CancelContactOutput::contact_id): <p>UUID of a contact.</p>
                            /// - On failure, responds with [`SdkError<CancelContactError>`](crate::error::CancelContactError)
    pub fn cancel_contact(&self) -> crate::client::fluent_builders::CancelContact {
                                crate::client::fluent_builders::CancelContact::new(self.handle.clone())
                            }
}

