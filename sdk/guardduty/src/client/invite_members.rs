// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`InviteMembers`](crate::client::fluent_builders::InviteMembers) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`detector_id(impl Into<String>)`](crate::client::fluent_builders::InviteMembers::detector_id) / [`set_detector_id(Option<String>)`](crate::client::fluent_builders::InviteMembers::set_detector_id): <p>The unique ID of the detector of the GuardDuty account that you want to invite members with.</p>
    ///   - [`account_ids(Vec<String>)`](crate::client::fluent_builders::InviteMembers::account_ids) / [`set_account_ids(Option<Vec<String>>)`](crate::client::fluent_builders::InviteMembers::set_account_ids): <p>A list of account IDs of the accounts that you want to invite to GuardDuty as members.</p>
    ///   - [`disable_email_notification(bool)`](crate::client::fluent_builders::InviteMembers::disable_email_notification) / [`set_disable_email_notification(bool)`](crate::client::fluent_builders::InviteMembers::set_disable_email_notification): <p>A Boolean value that specifies whether you want to disable email notification to the accounts that you are inviting to GuardDuty as members.</p>
    ///   - [`message(impl Into<String>)`](crate::client::fluent_builders::InviteMembers::message) / [`set_message(Option<String>)`](crate::client::fluent_builders::InviteMembers::set_message): <p>The invitation message that you want to send to the accounts that you're inviting to GuardDuty as members.</p>
                            /// - On success, responds with [`InviteMembersOutput`](crate::output::InviteMembersOutput) with field(s):
    ///   - [`unprocessed_accounts(Option<Vec<UnprocessedAccount>>)`](crate::output::InviteMembersOutput::unprocessed_accounts): <p>A list of objects that contain the unprocessed account and a result string that explains why it was unprocessed.</p>
                            /// - On failure, responds with [`SdkError<InviteMembersError>`](crate::error::InviteMembersError)
    pub fn invite_members(&self) -> crate::client::fluent_builders::InviteMembers {
                                crate::client::fluent_builders::InviteMembers::new(self.handle.clone())
                            }
}

