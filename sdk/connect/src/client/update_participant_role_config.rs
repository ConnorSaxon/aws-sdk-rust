// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateParticipantRoleConfig`](crate::client::fluent_builders::UpdateParticipantRoleConfig) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::UpdateParticipantRoleConfig::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::UpdateParticipantRoleConfig::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`contact_id(impl Into<String>)`](crate::client::fluent_builders::UpdateParticipantRoleConfig::contact_id) / [`set_contact_id(Option<String>)`](crate::client::fluent_builders::UpdateParticipantRoleConfig::set_contact_id): <p>The identifier of the contact in this instance of Amazon Connect. </p>
    ///   - [`channel_configuration(UpdateParticipantRoleConfigChannelInfo)`](crate::client::fluent_builders::UpdateParticipantRoleConfig::channel_configuration) / [`set_channel_configuration(Option<UpdateParticipantRoleConfigChannelInfo>)`](crate::client::fluent_builders::UpdateParticipantRoleConfig::set_channel_configuration): <p>The Amazon Connect channel you want to configure.</p>
                            /// - On success, responds with [`UpdateParticipantRoleConfigOutput`](crate::output::UpdateParticipantRoleConfigOutput)
                            /// - On failure, responds with [`SdkError<UpdateParticipantRoleConfigError>`](crate::error::UpdateParticipantRoleConfigError)
    pub fn update_participant_role_config(&self) -> crate::client::fluent_builders::UpdateParticipantRoleConfig {
                                crate::client::fluent_builders::UpdateParticipantRoleConfig::new(self.handle.clone())
                            }
}

