// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CloseInstancePublicPorts`](crate::client::fluent_builders::CloseInstancePublicPorts) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`port_info(PortInfo)`](crate::client::fluent_builders::CloseInstancePublicPorts::port_info) / [`set_port_info(Option<PortInfo>)`](crate::client::fluent_builders::CloseInstancePublicPorts::set_port_info): <p>An object to describe the ports to close for the specified instance.</p>
    ///   - [`instance_name(impl Into<String>)`](crate::client::fluent_builders::CloseInstancePublicPorts::instance_name) / [`set_instance_name(Option<String>)`](crate::client::fluent_builders::CloseInstancePublicPorts::set_instance_name): <p>The name of the instance for which to close ports.</p>
                            /// - On success, responds with [`CloseInstancePublicPortsOutput`](crate::output::CloseInstancePublicPortsOutput) with field(s):
    ///   - [`operation(Option<Operation>)`](crate::output::CloseInstancePublicPortsOutput::operation): <p>An object that describes the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
                            /// - On failure, responds with [`SdkError<CloseInstancePublicPortsError>`](crate::error::CloseInstancePublicPortsError)
    pub fn close_instance_public_ports(&self) -> crate::client::fluent_builders::CloseInstancePublicPorts {
                                crate::client::fluent_builders::CloseInstancePublicPorts::new(self.handle.clone())
                            }
}

