// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AddNotificationChannel`](crate::client::fluent_builders::AddNotificationChannel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`config(NotificationChannelConfig)`](crate::client::fluent_builders::AddNotificationChannel::config) / [`set_config(Option<NotificationChannelConfig>)`](crate::client::fluent_builders::AddNotificationChannel::set_config): <p> A <code>NotificationChannelConfig</code> object that specifies what type of notification channel to add. The one supported notification channel is Amazon Simple Notification Service (Amazon SNS). </p>
                            /// - On success, responds with [`AddNotificationChannelOutput`](crate::output::AddNotificationChannelOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::AddNotificationChannelOutput::id): <p> The ID of the added notification channel. </p>
                            /// - On failure, responds with [`SdkError<AddNotificationChannelError>`](crate::error::AddNotificationChannelError)
    pub fn add_notification_channel(&self) -> crate::client::fluent_builders::AddNotificationChannel {
                                crate::client::fluent_builders::AddNotificationChannel::new(self.handle.clone())
                            }
}

