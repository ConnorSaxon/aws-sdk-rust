// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociatePhoneNumbersWithVoiceConnectorGroup`](crate::client::fluent_builders::AssociatePhoneNumbersWithVoiceConnectorGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`voice_connector_group_id(impl Into<String>)`](crate::client::fluent_builders::AssociatePhoneNumbersWithVoiceConnectorGroup::voice_connector_group_id) / [`set_voice_connector_group_id(Option<String>)`](crate::client::fluent_builders::AssociatePhoneNumbersWithVoiceConnectorGroup::set_voice_connector_group_id): <p>The Amazon Chime Voice Connector group ID.</p>
    ///   - [`e164_phone_numbers(Vec<String>)`](crate::client::fluent_builders::AssociatePhoneNumbersWithVoiceConnectorGroup::e164_phone_numbers) / [`set_e164_phone_numbers(Option<Vec<String>>)`](crate::client::fluent_builders::AssociatePhoneNumbersWithVoiceConnectorGroup::set_e164_phone_numbers): <p>List of phone numbers, in E.164 format.</p>
    ///   - [`force_associate(bool)`](crate::client::fluent_builders::AssociatePhoneNumbersWithVoiceConnectorGroup::force_associate) / [`set_force_associate(Option<bool>)`](crate::client::fluent_builders::AssociatePhoneNumbersWithVoiceConnectorGroup::set_force_associate): <p>If true, associates the provided phone numbers with the provided Amazon Chime Voice Connector Group and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p>
                            /// - On success, responds with [`AssociatePhoneNumbersWithVoiceConnectorGroupOutput`](crate::output::AssociatePhoneNumbersWithVoiceConnectorGroupOutput) with field(s):
    ///   - [`phone_number_errors(Option<Vec<PhoneNumberError>>)`](crate::output::AssociatePhoneNumbersWithVoiceConnectorGroupOutput::phone_number_errors): <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
                            /// - On failure, responds with [`SdkError<AssociatePhoneNumbersWithVoiceConnectorGroupError>`](crate::error::AssociatePhoneNumbersWithVoiceConnectorGroupError)
    pub fn associate_phone_numbers_with_voice_connector_group(&self) -> crate::client::fluent_builders::AssociatePhoneNumbersWithVoiceConnectorGroup {
                                crate::client::fluent_builders::AssociatePhoneNumbersWithVoiceConnectorGroup::new(self.handle.clone())
                            }
}

