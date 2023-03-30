// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AcceptAdministratorInvitation`](crate::client::fluent_builders::AcceptAdministratorInvitation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`detector_id(impl Into<String>)`](crate::client::fluent_builders::AcceptAdministratorInvitation::detector_id) / [`set_detector_id(Option<String>)`](crate::client::fluent_builders::AcceptAdministratorInvitation::set_detector_id): <p>The unique ID of the detector of the GuardDuty member account.</p>
    ///   - [`administrator_id(impl Into<String>)`](crate::client::fluent_builders::AcceptAdministratorInvitation::administrator_id) / [`set_administrator_id(Option<String>)`](crate::client::fluent_builders::AcceptAdministratorInvitation::set_administrator_id): <p>The account ID of the GuardDuty administrator account whose invitation you're accepting.</p>
    ///   - [`invitation_id(impl Into<String>)`](crate::client::fluent_builders::AcceptAdministratorInvitation::invitation_id) / [`set_invitation_id(Option<String>)`](crate::client::fluent_builders::AcceptAdministratorInvitation::set_invitation_id): <p>The value that is used to validate the administrator account to the member account.</p>
                            /// - On success, responds with [`AcceptAdministratorInvitationOutput`](crate::output::AcceptAdministratorInvitationOutput)
                            /// - On failure, responds with [`SdkError<AcceptAdministratorInvitationError>`](crate::error::AcceptAdministratorInvitationError)
    pub fn accept_administrator_invitation(&self) -> crate::client::fluent_builders::AcceptAdministratorInvitation {
                                crate::client::fluent_builders::AcceptAdministratorInvitation::new(self.handle.clone())
                            }
}

