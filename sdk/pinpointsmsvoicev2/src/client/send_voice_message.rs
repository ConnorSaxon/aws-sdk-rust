// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendVoiceMessage`](crate::client::fluent_builders::SendVoiceMessage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`destination_phone_number(impl Into<String>)`](crate::client::fluent_builders::SendVoiceMessage::destination_phone_number) / [`set_destination_phone_number(Option<String>)`](crate::client::fluent_builders::SendVoiceMessage::set_destination_phone_number): <p>The destination phone number in E.164 format.</p>
    ///   - [`origination_identity(impl Into<String>)`](crate::client::fluent_builders::SendVoiceMessage::origination_identity) / [`set_origination_identity(Option<String>)`](crate::client::fluent_builders::SendVoiceMessage::set_origination_identity): <p>The origination identity to use for the voice call. This can be the PhoneNumber, PhoneNumberId, PhoneNumberArn, PoolId, or PoolArn.</p>
    ///   - [`message_body(impl Into<String>)`](crate::client::fluent_builders::SendVoiceMessage::message_body) / [`set_message_body(Option<String>)`](crate::client::fluent_builders::SendVoiceMessage::set_message_body): <p>The text to convert to a voice message.</p>
    ///   - [`message_body_text_type(VoiceMessageBodyTextType)`](crate::client::fluent_builders::SendVoiceMessage::message_body_text_type) / [`set_message_body_text_type(Option<VoiceMessageBodyTextType>)`](crate::client::fluent_builders::SendVoiceMessage::set_message_body_text_type): <p>Specifies if the MessageBody field contains text or <a href="https://docs.aws.amazon.com/polly/latest/dg/what-is.html">speech synthesis markup language (SSML)</a>.</p>  <ul>   <li> <p>TEXT: This is the default value. When used the maximum character limit is 3000.</p> </li>   <li> <p>SSML: When used the maximum character limit is 6000 including SSML tagging.</p> </li>  </ul>
    ///   - [`voice_id(VoiceId)`](crate::client::fluent_builders::SendVoiceMessage::voice_id) / [`set_voice_id(Option<VoiceId>)`](crate::client::fluent_builders::SendVoiceMessage::set_voice_id): <p>The voice for the <a href="https://docs.aws.amazon.com/polly/latest/dg/what-is.html">Amazon Polly</a> service to use. By default this is set to "MATTHEW".</p>
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::SendVoiceMessage::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::SendVoiceMessage::set_configuration_set_name): <p>The name of the configuration set to use. This can be either the ConfigurationSetName or ConfigurationSetArn.</p>
    ///   - [`max_price_per_minute(impl Into<String>)`](crate::client::fluent_builders::SendVoiceMessage::max_price_per_minute) / [`set_max_price_per_minute(Option<String>)`](crate::client::fluent_builders::SendVoiceMessage::set_max_price_per_minute): <p>The maximum amount to spend per voice message, in US dollars.</p>
    ///   - [`time_to_live(i32)`](crate::client::fluent_builders::SendVoiceMessage::time_to_live) / [`set_time_to_live(Option<i32>)`](crate::client::fluent_builders::SendVoiceMessage::set_time_to_live): <p>How long the voice message is valid for. By default this is 72 hours.</p>
    ///   - [`context(HashMap<String, String>)`](crate::client::fluent_builders::SendVoiceMessage::context) / [`set_context(Option<HashMap<String, String>>)`](crate::client::fluent_builders::SendVoiceMessage::set_context): <p>You can specify custom data in this field. If you do, that data is logged to the event destination.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::SendVoiceMessage::dry_run) / [`set_dry_run(bool)`](crate::client::fluent_builders::SendVoiceMessage::set_dry_run): <p>When set to true, the message is checked and validated, but isn't sent to the end recipient.</p>
                            /// - On success, responds with [`SendVoiceMessageOutput`](crate::output::SendVoiceMessageOutput) with field(s):
    ///   - [`message_id(Option<String>)`](crate::output::SendVoiceMessageOutput::message_id): <p>The unique identifier for the message.</p>
                            /// - On failure, responds with [`SdkError<SendVoiceMessageError>`](crate::error::SendVoiceMessageError)
    pub fn send_voice_message(&self) -> crate::client::fluent_builders::SendVoiceMessage {
                                crate::client::fluent_builders::SendVoiceMessage::new(self.handle.clone())
                            }
}

