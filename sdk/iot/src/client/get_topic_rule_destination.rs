// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetTopicRuleDestination`](crate::client::fluent_builders::GetTopicRuleDestination) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::GetTopicRuleDestination::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::GetTopicRuleDestination::set_arn): <p>The ARN of the topic rule destination.</p>
                            /// - On success, responds with [`GetTopicRuleDestinationOutput`](crate::output::GetTopicRuleDestinationOutput) with field(s):
    ///   - [`topic_rule_destination(Option<TopicRuleDestination>)`](crate::output::GetTopicRuleDestinationOutput::topic_rule_destination): <p>The topic rule destination.</p>
                            /// - On failure, responds with [`SdkError<GetTopicRuleDestinationError>`](crate::error::GetTopicRuleDestinationError)
    pub fn get_topic_rule_destination(&self) -> crate::client::fluent_builders::GetTopicRuleDestination {
                                crate::client::fluent_builders::GetTopicRuleDestination::new(self.handle.clone())
                            }
}

