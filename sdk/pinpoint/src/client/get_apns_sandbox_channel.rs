// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetApnsSandboxChannel`](crate::client::fluent_builders::GetApnsSandboxChannel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::GetApnsSandboxChannel::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::GetApnsSandboxChannel::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
                            /// - On success, responds with [`GetApnsSandboxChannelOutput`](crate::output::GetApnsSandboxChannelOutput) with field(s):
    ///   - [`apns_sandbox_channel_response(Option<ApnsSandboxChannelResponse>)`](crate::output::GetApnsSandboxChannelOutput::apns_sandbox_channel_response): <p>Provides information about the status and settings of the APNs (Apple Push Notification service) sandbox channel for an application.</p>
                            /// - On failure, responds with [`SdkError<GetApnsSandboxChannelError>`](crate::error::GetApnsSandboxChannelError)
    pub fn get_apns_sandbox_channel(&self) -> crate::client::fluent_builders::GetApnsSandboxChannel {
                                crate::client::fluent_builders::GetApnsSandboxChannel::new(self.handle.clone())
                            }
}

