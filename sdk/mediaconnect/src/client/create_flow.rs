// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateFlow`](crate::client::fluent_builders::CreateFlow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`availability_zone(impl Into<String>)`](crate::client::fluent_builders::CreateFlow::availability_zone) / [`set_availability_zone(Option<String>)`](crate::client::fluent_builders::CreateFlow::set_availability_zone): The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current AWS Region.
    ///   - [`entitlements(Vec<GrantEntitlementRequest>)`](crate::client::fluent_builders::CreateFlow::entitlements) / [`set_entitlements(Option<Vec<GrantEntitlementRequest>>)`](crate::client::fluent_builders::CreateFlow::set_entitlements): The entitlements that you want to grant on a flow.
    ///   - [`media_streams(Vec<AddMediaStreamRequest>)`](crate::client::fluent_builders::CreateFlow::media_streams) / [`set_media_streams(Option<Vec<AddMediaStreamRequest>>)`](crate::client::fluent_builders::CreateFlow::set_media_streams): The media streams that you want to add to the flow. You can associate these media streams with sources and outputs on the flow.
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateFlow::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateFlow::set_name): The name of the flow.
    ///   - [`outputs(Vec<AddOutputRequest>)`](crate::client::fluent_builders::CreateFlow::outputs) / [`set_outputs(Option<Vec<AddOutputRequest>>)`](crate::client::fluent_builders::CreateFlow::set_outputs): The outputs that you want to add to this flow.
    ///   - [`source(SetSourceRequest)`](crate::client::fluent_builders::CreateFlow::source) / [`set_source(Option<SetSourceRequest>)`](crate::client::fluent_builders::CreateFlow::set_source): The settings for the source of the flow.
    ///   - [`source_failover_config(FailoverConfig)`](crate::client::fluent_builders::CreateFlow::source_failover_config) / [`set_source_failover_config(Option<FailoverConfig>)`](crate::client::fluent_builders::CreateFlow::set_source_failover_config): The settings for source failover.
    ///   - [`sources(Vec<SetSourceRequest>)`](crate::client::fluent_builders::CreateFlow::sources) / [`set_sources(Option<Vec<SetSourceRequest>>)`](crate::client::fluent_builders::CreateFlow::set_sources): (undocumented)
    ///   - [`vpc_interfaces(Vec<VpcInterfaceRequest>)`](crate::client::fluent_builders::CreateFlow::vpc_interfaces) / [`set_vpc_interfaces(Option<Vec<VpcInterfaceRequest>>)`](crate::client::fluent_builders::CreateFlow::set_vpc_interfaces): The VPC interfaces you want on the flow.
    ///   - [`maintenance(AddMaintenance)`](crate::client::fluent_builders::CreateFlow::maintenance) / [`set_maintenance(Option<AddMaintenance>)`](crate::client::fluent_builders::CreateFlow::set_maintenance): Create maintenance setting for a flow
                            /// - On success, responds with [`CreateFlowOutput`](crate::output::CreateFlowOutput) with field(s):
    ///   - [`flow(Option<Flow>)`](crate::output::CreateFlowOutput::flow): The settings for a flow, including its source, outputs, and entitlements.
                            /// - On failure, responds with [`SdkError<CreateFlowError>`](crate::error::CreateFlowError)
    pub fn create_flow(&self) -> crate::client::fluent_builders::CreateFlow {
                                crate::client::fluent_builders::CreateFlow::new(self.handle.clone())
                            }
}

