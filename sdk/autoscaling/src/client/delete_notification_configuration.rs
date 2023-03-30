// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteNotificationConfiguration`](crate::client::fluent_builders::DeleteNotificationConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`auto_scaling_group_name(impl Into<String>)`](crate::client::fluent_builders::DeleteNotificationConfiguration::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::client::fluent_builders::DeleteNotificationConfiguration::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`topic_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteNotificationConfiguration::topic_arn) / [`set_topic_arn(Option<String>)`](crate::client::fluent_builders::DeleteNotificationConfiguration::set_topic_arn): <p>The Amazon Resource Name (ARN) of the Amazon SNS topic.</p>
                            /// - On success, responds with [`DeleteNotificationConfigurationOutput`](crate::output::DeleteNotificationConfigurationOutput)
                            /// - On failure, responds with [`SdkError<DeleteNotificationConfigurationError>`](crate::error::DeleteNotificationConfigurationError)
    pub fn delete_notification_configuration(&self) -> crate::client::fluent_builders::DeleteNotificationConfiguration {
                                crate::client::fluent_builders::DeleteNotificationConfiguration::new(self.handle.clone())
                            }
}

