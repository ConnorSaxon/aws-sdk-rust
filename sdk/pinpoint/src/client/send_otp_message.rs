// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendOTPMessage`](crate::client::fluent_builders::SendOTPMessage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::SendOTPMessage::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::SendOTPMessage::set_application_id): <p>The unique ID of your Amazon Pinpoint application.</p>
    ///   - [`send_otp_message_request_parameters(SendOtpMessageRequestParameters)`](crate::client::fluent_builders::SendOTPMessage::send_otp_message_request_parameters) / [`set_send_otp_message_request_parameters(Option<SendOtpMessageRequestParameters>)`](crate::client::fluent_builders::SendOTPMessage::set_send_otp_message_request_parameters): <p>Send OTP message request parameters.</p>
                            /// - On success, responds with [`SendOtpMessageOutput`](crate::output::SendOtpMessageOutput) with field(s):
    ///   - [`message_response(Option<MessageResponse>)`](crate::output::SendOtpMessageOutput::message_response): <p>Provides information about the results of a request to send a message to an endpoint address.</p>
                            /// - On failure, responds with [`SdkError<SendOTPMessageError>`](crate::error::SendOTPMessageError)
    pub fn send_otp_message(&self) -> crate::client::fluent_builders::SendOTPMessage {
                                crate::client::fluent_builders::SendOTPMessage::new(self.handle.clone())
                            }
}

