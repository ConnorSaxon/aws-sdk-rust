// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ConfirmSubscription`](crate::client::fluent_builders::ConfirmSubscription) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`topic_arn(impl Into<String>)`](crate::client::fluent_builders::ConfirmSubscription::topic_arn) / [`set_topic_arn(Option<String>)`](crate::client::fluent_builders::ConfirmSubscription::set_topic_arn): <p>The ARN of the topic for which you wish to confirm a subscription.</p>
    ///   - [`token(impl Into<String>)`](crate::client::fluent_builders::ConfirmSubscription::token) / [`set_token(Option<String>)`](crate::client::fluent_builders::ConfirmSubscription::set_token): <p>Short-lived token sent to an endpoint during the <code>Subscribe</code> action.</p>
    ///   - [`authenticate_on_unsubscribe(impl Into<String>)`](crate::client::fluent_builders::ConfirmSubscription::authenticate_on_unsubscribe) / [`set_authenticate_on_unsubscribe(Option<String>)`](crate::client::fluent_builders::ConfirmSubscription::set_authenticate_on_unsubscribe): <p>Disallows unauthenticated unsubscribes of the subscription. If the value of this parameter is <code>true</code> and the request has an Amazon Web Services signature, then only the topic owner and the subscription owner can unsubscribe the endpoint. The unsubscribe action requires Amazon Web Services authentication. </p>
                            /// - On success, responds with [`ConfirmSubscriptionOutput`](crate::output::ConfirmSubscriptionOutput) with field(s):
    ///   - [`subscription_arn(Option<String>)`](crate::output::ConfirmSubscriptionOutput::subscription_arn): <p>The ARN of the created subscription.</p>
                            /// - On failure, responds with [`SdkError<ConfirmSubscriptionError>`](crate::error::ConfirmSubscriptionError)
    pub fn confirm_subscription(&self) -> crate::client::fluent_builders::ConfirmSubscription {
                                crate::client::fluent_builders::ConfirmSubscription::new(self.handle.clone())
                            }
}

