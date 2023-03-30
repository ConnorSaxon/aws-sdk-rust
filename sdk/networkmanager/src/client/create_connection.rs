// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateConnection`](crate::client::fluent_builders::CreateConnection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::global_network_id) / [`set_global_network_id(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_global_network_id): <p>The ID of the global network.</p>
    ///   - [`device_id(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::device_id) / [`set_device_id(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_device_id): <p>The ID of the first device in the connection.</p>
    ///   - [`connected_device_id(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::connected_device_id) / [`set_connected_device_id(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_connected_device_id): <p>The ID of the second device in the connection.</p>
    ///   - [`link_id(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::link_id) / [`set_link_id(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_link_id): <p>The ID of the link for the first device.</p>
    ///   - [`connected_link_id(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::connected_link_id) / [`set_connected_link_id(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_connected_link_id): <p>The ID of the link for the second device.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateConnection::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateConnection::set_description): <p>A description of the connection.</p>  <p>Length Constraints: Maximum length of 256 characters.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateConnection::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateConnection::set_tags): <p>The tags to apply to the resource during creation.</p>
                            /// - On success, responds with [`CreateConnectionOutput`](crate::output::CreateConnectionOutput) with field(s):
    ///   - [`connection(Option<Connection>)`](crate::output::CreateConnectionOutput::connection): <p>Information about the connection.</p>
                            /// - On failure, responds with [`SdkError<CreateConnectionError>`](crate::error::CreateConnectionError)
    pub fn create_connection(&self) -> crate::client::fluent_builders::CreateConnection {
                                crate::client::fluent_builders::CreateConnection::new(self.handle.clone())
                            }
}

