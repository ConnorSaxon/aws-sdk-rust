// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateSubscriptionDefinitionVersion`](crate::client::fluent_builders::CreateSubscriptionDefinitionVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`amzn_client_token(impl Into<String>)`](crate::client::fluent_builders::CreateSubscriptionDefinitionVersion::amzn_client_token) / [`set_amzn_client_token(Option<String>)`](crate::client::fluent_builders::CreateSubscriptionDefinitionVersion::set_amzn_client_token): A client token used to correlate requests and responses.
    ///   - [`subscription_definition_id(impl Into<String>)`](crate::client::fluent_builders::CreateSubscriptionDefinitionVersion::subscription_definition_id) / [`set_subscription_definition_id(Option<String>)`](crate::client::fluent_builders::CreateSubscriptionDefinitionVersion::set_subscription_definition_id): The ID of the subscription definition.
    ///   - [`subscriptions(Vec<Subscription>)`](crate::client::fluent_builders::CreateSubscriptionDefinitionVersion::subscriptions) / [`set_subscriptions(Option<Vec<Subscription>>)`](crate::client::fluent_builders::CreateSubscriptionDefinitionVersion::set_subscriptions): A list of subscriptions.
                            /// - On success, responds with [`CreateSubscriptionDefinitionVersionOutput`](crate::output::CreateSubscriptionDefinitionVersionOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::CreateSubscriptionDefinitionVersionOutput::arn): The ARN of the version.
    ///   - [`creation_timestamp(Option<String>)`](crate::output::CreateSubscriptionDefinitionVersionOutput::creation_timestamp): The time, in milliseconds since the epoch, when the version was created.
    ///   - [`id(Option<String>)`](crate::output::CreateSubscriptionDefinitionVersionOutput::id): The ID of the parent definition that the version is associated with.
    ///   - [`version(Option<String>)`](crate::output::CreateSubscriptionDefinitionVersionOutput::version): The ID of the version.
                            /// - On failure, responds with [`SdkError<CreateSubscriptionDefinitionVersionError>`](crate::error::CreateSubscriptionDefinitionVersionError)
    pub fn create_subscription_definition_version(&self) -> crate::client::fluent_builders::CreateSubscriptionDefinitionVersion {
                                crate::client::fluent_builders::CreateSubscriptionDefinitionVersion::new(self.handle.clone())
                            }
}

