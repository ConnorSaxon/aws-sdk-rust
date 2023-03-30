// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateSubscription`](crate::client::fluent_builders::CreateSubscription) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::CreateSubscription::send) it.
                            /// - On success, responds with [`CreateSubscriptionOutput`](crate::output::CreateSubscriptionOutput)
                            /// - On failure, responds with [`SdkError<CreateSubscriptionError>`](crate::error::CreateSubscriptionError)
    pub fn create_subscription(&self) -> crate::client::fluent_builders::CreateSubscription {
                                crate::client::fluent_builders::CreateSubscription::new(self.handle.clone())
                            }
}

