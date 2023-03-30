// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateCustomRoutingAccelerator`](crate::client::fluent_builders::CreateCustomRoutingAccelerator) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::set_name): <p>The name of a custom routing accelerator. The name can have a maximum of 64 characters, must contain only alphanumeric characters or hyphens (-), and must not begin or end with a hyphen.</p>
    ///   - [`ip_address_type(IpAddressType)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::ip_address_type) / [`set_ip_address_type(Option<IpAddressType>)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::set_ip_address_type): <p>The IP address type that an accelerator supports. For a custom routing accelerator, the value must be IPV4.</p>
    ///   - [`ip_addresses(Vec<String>)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::ip_addresses) / [`set_ip_addresses(Option<Vec<String>>)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::set_ip_addresses): <p>Optionally, if you've added your own IP address pool to Global Accelerator (BYOIP), you can choose an IPv4 address from your own pool to use for the accelerator's static IPv4 address when you create an accelerator. </p>  <p>After you bring an address range to Amazon Web Services, it appears in your account as an address pool. When you create an accelerator, you can assign one IPv4 address from your range to it. Global Accelerator assigns you a second static IPv4 address from an Amazon IP address range. If you bring two IPv4 address ranges to Amazon Web Services, you can assign one IPv4 address from each range to your accelerator. This restriction is because Global Accelerator assigns each address range to a different network zone, for high availability.</p>  <p>You can specify one or two addresses, separated by a space. Do not include the /32 suffix.</p>  <p>Note that you can't update IP addresses for an existing accelerator. To change them, you must create a new accelerator with the new addresses.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring your own IP addresses (BYOIP)</a> in the <i>Global Accelerator Developer Guide</i>.</p>
    ///   - [`enabled(bool)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::enabled) / [`set_enabled(Option<bool>)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::set_enabled): <p>Indicates whether an accelerator is enabled. The value is true or false. The default value is true. </p>  <p>If the value is set to true, an accelerator cannot be deleted. If set to false, the accelerator can be deleted.</p>
    ///   - [`idempotency_token(impl Into<String>)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::idempotency_token) / [`set_idempotency_token(Option<String>)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::set_idempotency_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateCustomRoutingAccelerator::set_tags): <p>Create tags for an accelerator.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in Global Accelerator</a> in the <i>Global Accelerator Developer Guide</i>.</p>
                            /// - On success, responds with [`CreateCustomRoutingAcceleratorOutput`](crate::output::CreateCustomRoutingAcceleratorOutput) with field(s):
    ///   - [`accelerator(Option<CustomRoutingAccelerator>)`](crate::output::CreateCustomRoutingAcceleratorOutput::accelerator): <p>The accelerator that is created.</p>
                            /// - On failure, responds with [`SdkError<CreateCustomRoutingAcceleratorError>`](crate::error::CreateCustomRoutingAcceleratorError)
    pub fn create_custom_routing_accelerator(&self) -> crate::client::fluent_builders::CreateCustomRoutingAccelerator {
                                crate::client::fluent_builders::CreateCustomRoutingAccelerator::new(self.handle.clone())
                            }
}

