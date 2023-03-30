// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDeviceFleet`](crate::client::fluent_builders::CreateDeviceFleet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`device_fleet_name(impl Into<String>)`](crate::client::fluent_builders::CreateDeviceFleet::device_fleet_name) / [`set_device_fleet_name(Option<String>)`](crate::client::fluent_builders::CreateDeviceFleet::set_device_fleet_name): <p>The name of the fleet that the device belongs to.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateDeviceFleet::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::CreateDeviceFleet::set_role_arn): <p>The Amazon Resource Name (ARN) that has access to Amazon Web Services Internet of Things (IoT).</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateDeviceFleet::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateDeviceFleet::set_description): <p>A description of the fleet.</p>
    ///   - [`output_config(EdgeOutputConfig)`](crate::client::fluent_builders::CreateDeviceFleet::output_config) / [`set_output_config(Option<EdgeOutputConfig>)`](crate::client::fluent_builders::CreateDeviceFleet::set_output_config): <p>The output configuration for storing sample data collected by the fleet.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateDeviceFleet::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateDeviceFleet::set_tags): <p>Creates tags for the specified fleet.</p>
    ///   - [`enable_iot_role_alias(bool)`](crate::client::fluent_builders::CreateDeviceFleet::enable_iot_role_alias) / [`set_enable_iot_role_alias(Option<bool>)`](crate::client::fluent_builders::CreateDeviceFleet::set_enable_iot_role_alias): <p>Whether to create an Amazon Web Services IoT Role Alias during device fleet creation. The name of the role alias generated will match this pattern: "SageMakerEdge-{DeviceFleetName}".</p>  <p>For example, if your device fleet is called "demo-fleet", the name of the role alias will be "SageMakerEdge-demo-fleet".</p>
                            /// - On success, responds with [`CreateDeviceFleetOutput`](crate::output::CreateDeviceFleetOutput)
                            /// - On failure, responds with [`SdkError<CreateDeviceFleetError>`](crate::error::CreateDeviceFleetError)
    pub fn create_device_fleet(&self) -> crate::client::fluent_builders::CreateDeviceFleet {
                                crate::client::fluent_builders::CreateDeviceFleet::new(self.handle.clone())
                            }
}

