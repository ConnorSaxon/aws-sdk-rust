// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateChannel`](crate::client::fluent_builders::CreateChannel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateChannel::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateChannel::set_description): A short text description of the Channel.
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::CreateChannel::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::CreateChannel::set_id): The ID of the Channel. The ID must be unique within the region and it cannot be changed after a Channel is created.
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateChannel::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateChannel::set_tags): A collection of tags associated with a resource
                            /// - On success, responds with [`CreateChannelOutput`](crate::output::CreateChannelOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::CreateChannelOutput::arn): The Amazon Resource Name (ARN) assigned to the Channel.
    ///   - [`description(Option<String>)`](crate::output::CreateChannelOutput::description): A short text description of the Channel.
    ///   - [`egress_access_logs(Option<EgressAccessLogs>)`](crate::output::CreateChannelOutput::egress_access_logs): Configure egress access logging.
    ///   - [`hls_ingest(Option<HlsIngest>)`](crate::output::CreateChannelOutput::hls_ingest): An HTTP Live Streaming (HLS) ingest resource configuration.
    ///   - [`id(Option<String>)`](crate::output::CreateChannelOutput::id): The ID of the Channel.
    ///   - [`ingress_access_logs(Option<IngressAccessLogs>)`](crate::output::CreateChannelOutput::ingress_access_logs): Configure ingress access logging.
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::CreateChannelOutput::tags): A collection of tags associated with a resource
                            /// - On failure, responds with [`SdkError<CreateChannelError>`](crate::error::CreateChannelError)
    pub fn create_channel(&self) -> crate::client::fluent_builders::CreateChannel {
                                crate::client::fluent_builders::CreateChannel::new(self.handle.clone())
                            }
}

