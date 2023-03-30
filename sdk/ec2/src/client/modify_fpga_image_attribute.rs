// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyFpgaImageAttribute`](crate::client::fluent_builders::ModifyFpgaImageAttribute) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`fpga_image_id(impl Into<String>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::fpga_image_id) / [`set_fpga_image_id(Option<String>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::set_fpga_image_id): <p>The ID of the AFI.</p>
    ///   - [`attribute(FpgaImageAttributeName)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::attribute) / [`set_attribute(Option<FpgaImageAttributeName>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::set_attribute): <p>The name of the attribute.</p>
    ///   - [`operation_type(OperationType)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::operation_type) / [`set_operation_type(Option<OperationType>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::set_operation_type): <p>The operation type.</p>
    ///   - [`user_ids(Vec<String>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::user_ids) / [`set_user_ids(Option<Vec<String>>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::set_user_ids): <p>The Amazon Web Services account IDs. This parameter is valid only when modifying the <code>loadPermission</code> attribute.</p>
    ///   - [`user_groups(Vec<String>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::user_groups) / [`set_user_groups(Option<Vec<String>>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::set_user_groups): <p>The user groups. This parameter is valid only when modifying the <code>loadPermission</code> attribute.</p>
    ///   - [`product_codes(Vec<String>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::product_codes) / [`set_product_codes(Option<Vec<String>>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::set_product_codes): <p>The product codes. After you add a product code to an AFI, it can't be removed. This parameter is valid only when modifying the <code>productCodes</code> attribute.</p>
    ///   - [`load_permission(LoadPermissionModifications)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::load_permission) / [`set_load_permission(Option<LoadPermissionModifications>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::set_load_permission): <p>The load permission for the AFI.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::set_description): <p>A description for the AFI.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::ModifyFpgaImageAttribute::set_name): <p>A name for the AFI.</p>
                            /// - On success, responds with [`ModifyFpgaImageAttributeOutput`](crate::output::ModifyFpgaImageAttributeOutput) with field(s):
    ///   - [`fpga_image_attribute(Option<FpgaImageAttribute>)`](crate::output::ModifyFpgaImageAttributeOutput::fpga_image_attribute): <p>Information about the attribute.</p>
                            /// - On failure, responds with [`SdkError<ModifyFpgaImageAttributeError>`](crate::error::ModifyFpgaImageAttributeError)
    pub fn modify_fpga_image_attribute(&self) -> crate::client::fluent_builders::ModifyFpgaImageAttribute {
                                crate::client::fluent_builders::ModifyFpgaImageAttribute::new(self.handle.clone())
                            }
}

