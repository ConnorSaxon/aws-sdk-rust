// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDatalakeExceptionsSubscription`](crate::client::fluent_builders::GetDatalakeExceptionsSubscription) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::GetDatalakeExceptionsSubscription::send) it.
                            /// - On success, responds with [`GetDatalakeExceptionsSubscriptionOutput`](crate::output::GetDatalakeExceptionsSubscriptionOutput) with field(s):
    ///   - [`protocol_and_notification_endpoint(Option<ProtocolAndNotificationEndpoint>)`](crate::output::GetDatalakeExceptionsSubscriptionOutput::protocol_and_notification_endpoint): <p>Retrieves the exception notification subscription information.</p>
                            /// - On failure, responds with [`SdkError<GetDatalakeExceptionsSubscriptionError>`](crate::error::GetDatalakeExceptionsSubscriptionError)
    pub fn get_datalake_exceptions_subscription(&self) -> crate::client::fluent_builders::GetDatalakeExceptionsSubscription {
                                crate::client::fluent_builders::GetDatalakeExceptionsSubscription::new(self.handle.clone())
                            }
}

