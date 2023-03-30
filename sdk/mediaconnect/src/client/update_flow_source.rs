// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateFlowSource`](crate::client::fluent_builders::UpdateFlowSource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`decryption(UpdateEncryption)`](crate::client::fluent_builders::UpdateFlowSource::decryption) / [`set_decryption(Option<UpdateEncryption>)`](crate::client::fluent_builders::UpdateFlowSource::set_decryption): The type of encryption used on the content ingested from this source.
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateFlowSource::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateFlowSource::set_description): A description for the source. This value is not used or seen outside of the current AWS Elemental MediaConnect account.
    ///   - [`entitlement_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateFlowSource::entitlement_arn) / [`set_entitlement_arn(Option<String>)`](crate::client::fluent_builders::UpdateFlowSource::set_entitlement_arn): The ARN of the entitlement that allows you to subscribe to this flow. The entitlement is set by the flow originator, and the ARN is generated as part of the originator's flow.
    ///   - [`flow_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateFlowSource::flow_arn) / [`set_flow_arn(Option<String>)`](crate::client::fluent_builders::UpdateFlowSource::set_flow_arn): The flow that is associated with the source that you want to update.
    ///   - [`ingest_port(i32)`](crate::client::fluent_builders::UpdateFlowSource::ingest_port) / [`set_ingest_port(i32)`](crate::client::fluent_builders::UpdateFlowSource::set_ingest_port): The port that the flow will be listening on for incoming content.
    ///   - [`max_bitrate(i32)`](crate::client::fluent_builders::UpdateFlowSource::max_bitrate) / [`set_max_bitrate(i32)`](crate::client::fluent_builders::UpdateFlowSource::set_max_bitrate): The smoothing max bitrate for RIST, RTP, and RTP-FEC streams.
    ///   - [`max_latency(i32)`](crate::client::fluent_builders::UpdateFlowSource::max_latency) / [`set_max_latency(i32)`](crate::client::fluent_builders::UpdateFlowSource::set_max_latency): The maximum latency in milliseconds. This parameter applies only to RIST-based, Zixi-based, and Fujitsu-based streams.
    ///   - [`max_sync_buffer(i32)`](crate::client::fluent_builders::UpdateFlowSource::max_sync_buffer) / [`set_max_sync_buffer(i32)`](crate::client::fluent_builders::UpdateFlowSource::set_max_sync_buffer): The size of the buffer (in milliseconds) to use to sync incoming source data.
    ///   - [`media_stream_source_configurations(Vec<MediaStreamSourceConfigurationRequest>)`](crate::client::fluent_builders::UpdateFlowSource::media_stream_source_configurations) / [`set_media_stream_source_configurations(Option<Vec<MediaStreamSourceConfigurationRequest>>)`](crate::client::fluent_builders::UpdateFlowSource::set_media_stream_source_configurations): The media streams that are associated with the source, and the parameters for those associations.
    ///   - [`min_latency(i32)`](crate::client::fluent_builders::UpdateFlowSource::min_latency) / [`set_min_latency(i32)`](crate::client::fluent_builders::UpdateFlowSource::set_min_latency): The minimum latency in milliseconds for SRT-based streams. In streams that use the SRT protocol, this value that you set on your MediaConnect source or output represents the minimal potential latency of that connection. The latency of the stream is set to the highest number between the sender’s minimum latency and the receiver’s minimum latency.
    ///   - [`protocol(Protocol)`](crate::client::fluent_builders::UpdateFlowSource::protocol) / [`set_protocol(Option<Protocol>)`](crate::client::fluent_builders::UpdateFlowSource::set_protocol): The protocol that is used by the source.
    ///   - [`sender_control_port(i32)`](crate::client::fluent_builders::UpdateFlowSource::sender_control_port) / [`set_sender_control_port(i32)`](crate::client::fluent_builders::UpdateFlowSource::set_sender_control_port): The port that the flow uses to send outbound requests to initiate connection with the sender.
    ///   - [`sender_ip_address(impl Into<String>)`](crate::client::fluent_builders::UpdateFlowSource::sender_ip_address) / [`set_sender_ip_address(Option<String>)`](crate::client::fluent_builders::UpdateFlowSource::set_sender_ip_address): The IP address that the flow communicates with to initiate connection with the sender.
    ///   - [`source_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateFlowSource::source_arn) / [`set_source_arn(Option<String>)`](crate::client::fluent_builders::UpdateFlowSource::set_source_arn): The ARN of the source that you want to update.
    ///   - [`source_listener_address(impl Into<String>)`](crate::client::fluent_builders::UpdateFlowSource::source_listener_address) / [`set_source_listener_address(Option<String>)`](crate::client::fluent_builders::UpdateFlowSource::set_source_listener_address): Source IP or domain name for SRT-caller protocol.
    ///   - [`source_listener_port(i32)`](crate::client::fluent_builders::UpdateFlowSource::source_listener_port) / [`set_source_listener_port(i32)`](crate::client::fluent_builders::UpdateFlowSource::set_source_listener_port): Source port for SRT-caller protocol.
    ///   - [`stream_id(impl Into<String>)`](crate::client::fluent_builders::UpdateFlowSource::stream_id) / [`set_stream_id(Option<String>)`](crate::client::fluent_builders::UpdateFlowSource::set_stream_id): The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.
    ///   - [`vpc_interface_name(impl Into<String>)`](crate::client::fluent_builders::UpdateFlowSource::vpc_interface_name) / [`set_vpc_interface_name(Option<String>)`](crate::client::fluent_builders::UpdateFlowSource::set_vpc_interface_name): The name of the VPC interface to use for this source.
    ///   - [`whitelist_cidr(impl Into<String>)`](crate::client::fluent_builders::UpdateFlowSource::whitelist_cidr) / [`set_whitelist_cidr(Option<String>)`](crate::client::fluent_builders::UpdateFlowSource::set_whitelist_cidr): The range of IP addresses that should be allowed to contribute content to your source. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.
                            /// - On success, responds with [`UpdateFlowSourceOutput`](crate::output::UpdateFlowSourceOutput) with field(s):
    ///   - [`flow_arn(Option<String>)`](crate::output::UpdateFlowSourceOutput::flow_arn): The ARN of the flow that you want to update.
    ///   - [`source(Option<Source>)`](crate::output::UpdateFlowSourceOutput::source): The settings for the source of the flow.
                            /// - On failure, responds with [`SdkError<UpdateFlowSourceError>`](crate::error::UpdateFlowSourceError)
    pub fn update_flow_source(&self) -> crate::client::fluent_builders::UpdateFlowSource {
                                crate::client::fluent_builders::UpdateFlowSource::new(self.handle.clone())
                            }
}

