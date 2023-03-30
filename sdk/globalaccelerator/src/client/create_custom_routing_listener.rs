// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateCustomRoutingListener`](crate::client::fluent_builders::CreateCustomRoutingListener) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accelerator_arn(impl Into<String>)`](crate::client::fluent_builders::CreateCustomRoutingListener::accelerator_arn) / [`set_accelerator_arn(Option<String>)`](crate::client::fluent_builders::CreateCustomRoutingListener::set_accelerator_arn): <p>The Amazon Resource Name (ARN) of the accelerator for a custom routing listener.</p>
    ///   - [`port_ranges(Vec<PortRange>)`](crate::client::fluent_builders::CreateCustomRoutingListener::port_ranges) / [`set_port_ranges(Option<Vec<PortRange>>)`](crate::client::fluent_builders::CreateCustomRoutingListener::set_port_ranges): <p>The port range to support for connections from clients to your accelerator.</p>  <p>Separately, you set port ranges for endpoints. For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/about-custom-routing-endpoints.html">About endpoints for custom routing accelerators</a>.</p>
    ///   - [`idempotency_token(impl Into<String>)`](crate::client::fluent_builders::CreateCustomRoutingListener::idempotency_token) / [`set_idempotency_token(Option<String>)`](crate::client::fluent_builders::CreateCustomRoutingListener::set_idempotency_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
                            /// - On success, responds with [`CreateCustomRoutingListenerOutput`](crate::output::CreateCustomRoutingListenerOutput) with field(s):
    ///   - [`listener(Option<CustomRoutingListener>)`](crate::output::CreateCustomRoutingListenerOutput::listener): <p>The listener that you've created for a custom routing accelerator.</p>
                            /// - On failure, responds with [`SdkError<CreateCustomRoutingListenerError>`](crate::error::CreateCustomRoutingListenerError)
    pub fn create_custom_routing_listener(&self) -> crate::client::fluent_builders::CreateCustomRoutingListener {
                                crate::client::fluent_builders::CreateCustomRoutingListener::new(self.handle.clone())
                            }
}

