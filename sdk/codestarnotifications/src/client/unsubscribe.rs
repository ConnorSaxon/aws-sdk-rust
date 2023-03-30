// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`Unsubscribe`](crate::client::fluent_builders::Unsubscribe) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::Unsubscribe::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::Unsubscribe::set_arn): <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    ///   - [`target_address(impl Into<String>)`](crate::client::fluent_builders::Unsubscribe::target_address) / [`set_target_address(Option<String>)`](crate::client::fluent_builders::Unsubscribe::set_target_address): <p>The ARN of the Chatbot topic to unsubscribe from the notification rule.</p>
                            /// - On success, responds with [`UnsubscribeOutput`](crate::output::UnsubscribeOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::UnsubscribeOutput::arn): <p>The Amazon Resource Name (ARN) of the the notification rule from which you have removed a subscription.</p>
                            /// - On failure, responds with [`SdkError<UnsubscribeError>`](crate::error::UnsubscribeError)
    pub fn unsubscribe(&self) -> crate::client::fluent_builders::Unsubscribe {
                                crate::client::fluent_builders::Unsubscribe::new(self.handle.clone())
                            }
}

