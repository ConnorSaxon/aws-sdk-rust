// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateChannelFlow`](crate::client::fluent_builders::CreateChannelFlow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_instance_arn(impl Into<String>)`](crate::client::fluent_builders::CreateChannelFlow::app_instance_arn) / [`set_app_instance_arn(Option<String>)`](crate::client::fluent_builders::CreateChannelFlow::set_app_instance_arn): <p>The ARN of the channel flow request.</p>
    ///   - [`processors(Vec<Processor>)`](crate::client::fluent_builders::CreateChannelFlow::processors) / [`set_processors(Option<Vec<Processor>>)`](crate::client::fluent_builders::CreateChannelFlow::set_processors): <p>Information about the processor Lambda functions.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateChannelFlow::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateChannelFlow::set_name): <p>The name of the channel flow.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateChannelFlow::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateChannelFlow::set_tags): <p>The tags for the creation request.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CreateChannelFlow::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CreateChannelFlow::set_client_request_token): <p>The client token for the request. An Idempotency token.</p>
                            /// - On success, responds with [`CreateChannelFlowOutput`](crate::output::CreateChannelFlowOutput) with field(s):
    ///   - [`channel_flow_arn(Option<String>)`](crate::output::CreateChannelFlowOutput::channel_flow_arn): <p>The ARN of the channel flow.</p>
                            /// - On failure, responds with [`SdkError<CreateChannelFlowError>`](crate::error::CreateChannelFlowError)
    pub fn create_channel_flow(&self) -> crate::client::fluent_builders::CreateChannelFlow {
                                crate::client::fluent_builders::CreateChannelFlow::new(self.handle.clone())
                            }
}

