// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetNotificationConfiguration`](crate::client::fluent_builders::GetNotificationConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`profiling_group_name(impl Into<String>)`](crate::client::fluent_builders::GetNotificationConfiguration::profiling_group_name) / [`set_profiling_group_name(Option<String>)`](crate::client::fluent_builders::GetNotificationConfiguration::set_profiling_group_name): <p>The name of the profiling group we want to get the notification configuration for.</p>
                            /// - On success, responds with [`GetNotificationConfigurationOutput`](crate::output::GetNotificationConfigurationOutput) with field(s):
    ///   - [`notification_configuration(Option<NotificationConfiguration>)`](crate::output::GetNotificationConfigurationOutput::notification_configuration): <p>The current notification configuration for this profiling group.</p>
                            /// - On failure, responds with [`SdkError<GetNotificationConfigurationError>`](crate::error::GetNotificationConfigurationError)
    pub fn get_notification_configuration(&self) -> crate::client::fluent_builders::GetNotificationConfiguration {
                                crate::client::fluent_builders::GetNotificationConfiguration::new(self.handle.clone())
                            }
}

