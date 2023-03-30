// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateEndpoint`](crate::client::fluent_builders::UpdateEndpoint) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateEndpoint::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateEndpoint::set_name): <p>The name of the endpoint you want to update.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateEndpoint::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateEndpoint::set_description): <p>A description for the endpoint.</p>
    ///   - [`routing_config(RoutingConfig)`](crate::client::fluent_builders::UpdateEndpoint::routing_config) / [`set_routing_config(Option<RoutingConfig>)`](crate::client::fluent_builders::UpdateEndpoint::set_routing_config): <p>Configure the routing policy, including the health check and secondary Region..</p>
    ///   - [`replication_config(ReplicationConfig)`](crate::client::fluent_builders::UpdateEndpoint::replication_config) / [`set_replication_config(Option<ReplicationConfig>)`](crate::client::fluent_builders::UpdateEndpoint::set_replication_config): <p>Whether event replication was enabled or disabled by this request.</p>
    ///   - [`event_buses(Vec<EndpointEventBus>)`](crate::client::fluent_builders::UpdateEndpoint::event_buses) / [`set_event_buses(Option<Vec<EndpointEventBus>>)`](crate::client::fluent_builders::UpdateEndpoint::set_event_buses): <p>Define event buses used for replication.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateEndpoint::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::UpdateEndpoint::set_role_arn): <p>The ARN of the role used by event replication for this request.</p>
                            /// - On success, responds with [`UpdateEndpointOutput`](crate::output::UpdateEndpointOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::UpdateEndpointOutput::name): <p>The name of the endpoint you updated in this request.</p>
    ///   - [`arn(Option<String>)`](crate::output::UpdateEndpointOutput::arn): <p>The ARN of the endpoint you updated in this request.</p>
    ///   - [`routing_config(Option<RoutingConfig>)`](crate::output::UpdateEndpointOutput::routing_config): <p>The routing configuration you updated in this request.</p>
    ///   - [`replication_config(Option<ReplicationConfig>)`](crate::output::UpdateEndpointOutput::replication_config): <p>Whether event replication was enabled or disabled for the endpoint you updated in this request.</p>
    ///   - [`event_buses(Option<Vec<EndpointEventBus>>)`](crate::output::UpdateEndpointOutput::event_buses): <p>The event buses used for replication for the endpoint you updated in this request.</p>
    ///   - [`role_arn(Option<String>)`](crate::output::UpdateEndpointOutput::role_arn): <p>The ARN of the role used by event replication for the endpoint you updated in this request.</p>
    ///   - [`endpoint_id(Option<String>)`](crate::output::UpdateEndpointOutput::endpoint_id): <p>The ID of the endpoint you updated in this request.</p>
    ///   - [`endpoint_url(Option<String>)`](crate::output::UpdateEndpointOutput::endpoint_url): <p>The URL of the endpoint you updated in this request.</p>
    ///   - [`state(Option<EndpointState>)`](crate::output::UpdateEndpointOutput::state): <p>The state of the endpoint you updated in this request.</p>
                            /// - On failure, responds with [`SdkError<UpdateEndpointError>`](crate::error::UpdateEndpointError)
    pub fn update_endpoint(&self) -> crate::client::fluent_builders::UpdateEndpoint {
                                crate::client::fluent_builders::UpdateEndpoint::new(self.handle.clone())
                            }
}

