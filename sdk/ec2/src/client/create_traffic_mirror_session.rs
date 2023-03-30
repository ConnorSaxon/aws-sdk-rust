// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateTrafficMirrorSession`](crate::client::fluent_builders::CreateTrafficMirrorSession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`network_interface_id(impl Into<String>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::network_interface_id) / [`set_network_interface_id(Option<String>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::set_network_interface_id): <p>The ID of the source network interface.</p>
    ///   - [`traffic_mirror_target_id(impl Into<String>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::traffic_mirror_target_id) / [`set_traffic_mirror_target_id(Option<String>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::set_traffic_mirror_target_id): <p>The ID of the Traffic Mirror target.</p>
    ///   - [`traffic_mirror_filter_id(impl Into<String>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::traffic_mirror_filter_id) / [`set_traffic_mirror_filter_id(Option<String>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::set_traffic_mirror_filter_id): <p>The ID of the Traffic Mirror filter.</p>
    ///   - [`packet_length(i32)`](crate::client::fluent_builders::CreateTrafficMirrorSession::packet_length) / [`set_packet_length(Option<i32>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::set_packet_length): <p>The number of bytes in each packet to mirror. These are bytes after the VXLAN header. Do not specify this parameter when you want to mirror the entire packet. To mirror a subset of the packet, set this to the length (in bytes) that you want to mirror. For example, if you set this value to 100, then the first 100 bytes that meet the filter criteria are copied to the target.</p>  <p>If you do not want to mirror the entire packet, use the <code>PacketLength</code> parameter to specify the number of bytes in each packet to mirror.</p>
    ///   - [`session_number(i32)`](crate::client::fluent_builders::CreateTrafficMirrorSession::session_number) / [`set_session_number(Option<i32>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::set_session_number): <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>  <p>Valid values are 1-32766.</p>
    ///   - [`virtual_network_id(i32)`](crate::client::fluent_builders::CreateTrafficMirrorSession::virtual_network_id) / [`set_virtual_network_id(Option<i32>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::set_virtual_network_id): <p>The VXLAN ID for the Traffic Mirror session. For more information about the VXLAN protocol, see <a href="https://tools.ietf.org/html/rfc7348">RFC 7348</a>. If you do not specify a <code>VirtualNetworkId</code>, an account-wide unique id is chosen at random.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::set_description): <p>The description of the Traffic Mirror session.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::set_tag_specifications): <p>The tags to assign to a Traffic Mirror session.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateTrafficMirrorSession::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateTrafficMirrorSession::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
                            /// - On success, responds with [`CreateTrafficMirrorSessionOutput`](crate::output::CreateTrafficMirrorSessionOutput) with field(s):
    ///   - [`traffic_mirror_session(Option<TrafficMirrorSession>)`](crate::output::CreateTrafficMirrorSessionOutput::traffic_mirror_session): <p>Information about the Traffic Mirror session.</p>
    ///   - [`client_token(Option<String>)`](crate::output::CreateTrafficMirrorSessionOutput::client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
                            /// - On failure, responds with [`SdkError<CreateTrafficMirrorSessionError>`](crate::error::CreateTrafficMirrorSessionError)
    pub fn create_traffic_mirror_session(&self) -> crate::client::fluent_builders::CreateTrafficMirrorSession {
                                crate::client::fluent_builders::CreateTrafficMirrorSession::new(self.handle.clone())
                            }
}

