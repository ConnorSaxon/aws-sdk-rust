// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AttachNetworkInterface`](crate::client::fluent_builders::AttachNetworkInterface) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`device_index(i32)`](crate::client::fluent_builders::AttachNetworkInterface::device_index) / [`set_device_index(Option<i32>)`](crate::client::fluent_builders::AttachNetworkInterface::set_device_index): <p>The index of the device for the network interface attachment.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::AttachNetworkInterface::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::AttachNetworkInterface::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::AttachNetworkInterface::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::AttachNetworkInterface::set_instance_id): <p>The ID of the instance.</p>
    ///   - [`network_interface_id(impl Into<String>)`](crate::client::fluent_builders::AttachNetworkInterface::network_interface_id) / [`set_network_interface_id(Option<String>)`](crate::client::fluent_builders::AttachNetworkInterface::set_network_interface_id): <p>The ID of the network interface.</p>
    ///   - [`network_card_index(i32)`](crate::client::fluent_builders::AttachNetworkInterface::network_card_index) / [`set_network_card_index(Option<i32>)`](crate::client::fluent_builders::AttachNetworkInterface::set_network_card_index): <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    ///   - [`ena_srd_specification(EnaSrdSpecification)`](crate::client::fluent_builders::AttachNetworkInterface::ena_srd_specification) / [`set_ena_srd_specification(Option<EnaSrdSpecification>)`](crate::client::fluent_builders::AttachNetworkInterface::set_ena_srd_specification): <p>Configures ENA Express for the network interface that this action attaches to the instance.</p>
                            /// - On success, responds with [`AttachNetworkInterfaceOutput`](crate::output::AttachNetworkInterfaceOutput) with field(s):
    ///   - [`attachment_id(Option<String>)`](crate::output::AttachNetworkInterfaceOutput::attachment_id): <p>The ID of the network interface attachment.</p>
    ///   - [`network_card_index(Option<i32>)`](crate::output::AttachNetworkInterfaceOutput::network_card_index): <p>The index of the network card.</p>
                            /// - On failure, responds with [`SdkError<AttachNetworkInterfaceError>`](crate::error::AttachNetworkInterfaceError)
    pub fn attach_network_interface(&self) -> crate::client::fluent_builders::AttachNetworkInterface {
                                crate::client::fluent_builders::AttachNetworkInterface::new(self.handle.clone())
                            }
}

