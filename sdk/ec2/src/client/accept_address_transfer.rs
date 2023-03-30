// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AcceptAddressTransfer`](crate::client::fluent_builders::AcceptAddressTransfer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`address(impl Into<String>)`](crate::client::fluent_builders::AcceptAddressTransfer::address) / [`set_address(Option<String>)`](crate::client::fluent_builders::AcceptAddressTransfer::set_address): <p>The Elastic IP address you are accepting for transfer.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::AcceptAddressTransfer::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::AcceptAddressTransfer::set_tag_specifications): <p> <code>tag</code>:<key>    - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key    <code>Owner</code> and the value    <code>TeamA</code>, specify    <code>tag:Owner</code> for the filter name and    <code>TeamA</code> for the filter value.  </key></p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::AcceptAddressTransfer::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::AcceptAddressTransfer::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`AcceptAddressTransferOutput`](crate::output::AcceptAddressTransferOutput) with field(s):
    ///   - [`address_transfer(Option<AddressTransfer>)`](crate::output::AcceptAddressTransferOutput::address_transfer): <p>An Elastic IP address transfer.</p>
                            /// - On failure, responds with [`SdkError<AcceptAddressTransferError>`](crate::error::AcceptAddressTransferError)
    pub fn accept_address_transfer(&self) -> crate::client::fluent_builders::AcceptAddressTransfer {
                                crate::client::fluent_builders::AcceptAddressTransfer::new(self.handle.clone())
                            }
}

