// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendTextMessage`](crate::client::fluent_builders::SendTextMessage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`destination_phone_number(impl Into<String>)`](crate::client::fluent_builders::SendTextMessage::destination_phone_number) / [`set_destination_phone_number(Option<String>)`](crate::client::fluent_builders::SendTextMessage::set_destination_phone_number): <p>The destination phone number in E.164 format.</p>
    ///   - [`origination_identity(impl Into<String>)`](crate::client::fluent_builders::SendTextMessage::origination_identity) / [`set_origination_identity(Option<String>)`](crate::client::fluent_builders::SendTextMessage::set_origination_identity): <p>The origination identity of the message. This can be either the PhoneNumber, PhoneNumberId, PhoneNumberArn, SenderId, SenderIdArn, PoolId, or PoolArn.</p>
    ///   - [`message_body(impl Into<String>)`](crate::client::fluent_builders::SendTextMessage::message_body) / [`set_message_body(Option<String>)`](crate::client::fluent_builders::SendTextMessage::set_message_body): <p>The body of the text message.</p>
    ///   - [`message_type(MessageType)`](crate::client::fluent_builders::SendTextMessage::message_type) / [`set_message_type(Option<MessageType>)`](crate::client::fluent_builders::SendTextMessage::set_message_type): <p>The type of message. Valid values are TRANSACTIONAL for messages that are critical or time-sensitive and PROMOTIONAL for messages that aren't critical or time-sensitive.</p>
    ///   - [`keyword(impl Into<String>)`](crate::client::fluent_builders::SendTextMessage::keyword) / [`set_keyword(Option<String>)`](crate::client::fluent_builders::SendTextMessage::set_keyword): <p>When you register a short code in the US, you must specify a program name. If you don’t have a US short code, omit this attribute.</p>
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::SendTextMessage::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::SendTextMessage::set_configuration_set_name): <p>The name of the configuration set to use. This can be either the ConfigurationSetName or ConfigurationSetArn.</p>
    ///   - [`max_price(impl Into<String>)`](crate::client::fluent_builders::SendTextMessage::max_price) / [`set_max_price(Option<String>)`](crate::client::fluent_builders::SendTextMessage::set_max_price): <p>The maximum amount that you want to spend, in US dollars, per each text message part. A text message can contain multiple parts.</p>
    ///   - [`time_to_live(i32)`](crate::client::fluent_builders::SendTextMessage::time_to_live) / [`set_time_to_live(Option<i32>)`](crate::client::fluent_builders::SendTextMessage::set_time_to_live): <p>How long the text message is valid for. By default this is 72 hours.</p>
    ///   - [`context(HashMap<String, String>)`](crate::client::fluent_builders::SendTextMessage::context) / [`set_context(Option<HashMap<String, String>>)`](crate::client::fluent_builders::SendTextMessage::set_context): <p>You can specify custom data in this field. If you do, that data is logged to the event destination.</p>
    ///   - [`destination_country_parameters(HashMap<DestinationCountryParameterKey, String>)`](crate::client::fluent_builders::SendTextMessage::destination_country_parameters) / [`set_destination_country_parameters(Option<HashMap<DestinationCountryParameterKey, String>>)`](crate::client::fluent_builders::SendTextMessage::set_destination_country_parameters): <p>This field is used for any country-specific registration requirements. Currently, this setting is only used when you send messages to recipients in India using a sender ID. For more information see <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/channels-sms-senderid-india.html">Special requirements for sending SMS messages to recipients in India</a>. </p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::SendTextMessage::dry_run) / [`set_dry_run(bool)`](crate::client::fluent_builders::SendTextMessage::set_dry_run): <p>When set to true, the message is checked and validated, but isn't sent to the end recipient.</p>
                            /// - On success, responds with [`SendTextMessageOutput`](crate::output::SendTextMessageOutput) with field(s):
    ///   - [`message_id(Option<String>)`](crate::output::SendTextMessageOutput::message_id): <p>The unique identifier for the message.</p>
                            /// - On failure, responds with [`SdkError<SendTextMessageError>`](crate::error::SendTextMessageError)
    pub fn send_text_message(&self) -> crate::client::fluent_builders::SendTextMessage {
                                crate::client::fluent_builders::SendTextMessage::new(self.handle.clone())
                            }
}

