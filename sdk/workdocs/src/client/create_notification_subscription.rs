// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateNotificationSubscription`](crate::client::fluent_builders::CreateNotificationSubscription) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::client::fluent_builders::CreateNotificationSubscription::organization_id) / [`set_organization_id(Option<String>)`](crate::client::fluent_builders::CreateNotificationSubscription::set_organization_id): <p>The ID of the organization.</p>
    ///   - [`endpoint(impl Into<String>)`](crate::client::fluent_builders::CreateNotificationSubscription::endpoint) / [`set_endpoint(Option<String>)`](crate::client::fluent_builders::CreateNotificationSubscription::set_endpoint): <p>The endpoint to receive the notifications. If the protocol is HTTPS, the endpoint is a URL that begins with <code>https</code>.</p>
    ///   - [`protocol(SubscriptionProtocolType)`](crate::client::fluent_builders::CreateNotificationSubscription::protocol) / [`set_protocol(Option<SubscriptionProtocolType>)`](crate::client::fluent_builders::CreateNotificationSubscription::set_protocol): <p>The protocol to use. The supported value is https, which delivers JSON-encoded messages using HTTPS POST.</p>
    ///   - [`subscription_type(SubscriptionType)`](crate::client::fluent_builders::CreateNotificationSubscription::subscription_type) / [`set_subscription_type(Option<SubscriptionType>)`](crate::client::fluent_builders::CreateNotificationSubscription::set_subscription_type): <p>The notification type.</p>
                            /// - On success, responds with [`CreateNotificationSubscriptionOutput`](crate::output::CreateNotificationSubscriptionOutput) with field(s):
    ///   - [`subscription(Option<Subscription>)`](crate::output::CreateNotificationSubscriptionOutput::subscription): <p>The subscription.</p>
                            /// - On failure, responds with [`SdkError<CreateNotificationSubscriptionError>`](crate::error::CreateNotificationSubscriptionError)
    pub fn create_notification_subscription(&self) -> crate::client::fluent_builders::CreateNotificationSubscription {
                                crate::client::fluent_builders::CreateNotificationSubscription::new(self.handle.clone())
                            }
}

