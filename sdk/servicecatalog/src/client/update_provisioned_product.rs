// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateProvisionedProduct`](crate::client::fluent_builders::UpdateProvisionedProduct) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accept_language(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::accept_language) / [`set_accept_language(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_accept_language): <p>The language code.</p>  <ul>   <li> <p> <code>en</code> - English (default)</p> </li>   <li> <p> <code>jp</code> - Japanese</p> </li>   <li> <p> <code>zh</code> - Chinese</p> </li>  </ul>
    ///   - [`provisioned_product_name(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::provisioned_product_name) / [`set_provisioned_product_name(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_provisioned_product_name): <p>The name of the provisioned product. You cannot specify both <code>ProvisionedProductName</code> and <code>ProvisionedProductId</code>.</p>
    ///   - [`provisioned_product_id(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::provisioned_product_id) / [`set_provisioned_product_id(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_provisioned_product_id): <p>The identifier of the provisioned product. You must provide the name or ID, but not both.</p>
    ///   - [`product_id(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::product_id) / [`set_product_id(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_product_id): <p>The identifier of the product. You must provide the name or ID, but not both.</p>
    ///   - [`product_name(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::product_name) / [`set_product_name(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_product_name): <p>The name of the product. You must provide the name or ID, but not both.</p>
    ///   - [`provisioning_artifact_id(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::provisioning_artifact_id) / [`set_provisioning_artifact_id(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_provisioning_artifact_id): <p>The identifier of the provisioning artifact.</p>
    ///   - [`provisioning_artifact_name(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::provisioning_artifact_name) / [`set_provisioning_artifact_name(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_provisioning_artifact_name): <p>The name of the provisioning artifact. You must provide the name or ID, but not both.</p>
    ///   - [`path_id(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::path_id) / [`set_path_id(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_path_id): <p>The path identifier. This value is optional if the product has a default path, and required if the product has more than one path. You must provide the name or ID, but not both.</p>
    ///   - [`path_name(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::path_name) / [`set_path_name(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_path_name): <p>The name of the path. You must provide the name or ID, but not both.</p>
    ///   - [`provisioning_parameters(Vec<UpdateProvisioningParameter>)`](crate::client::fluent_builders::UpdateProvisionedProduct::provisioning_parameters) / [`set_provisioning_parameters(Option<Vec<UpdateProvisioningParameter>>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_provisioning_parameters): <p>The new parameters.</p>
    ///   - [`provisioning_preferences(UpdateProvisioningPreferences)`](crate::client::fluent_builders::UpdateProvisionedProduct::provisioning_preferences) / [`set_provisioning_preferences(Option<UpdateProvisioningPreferences>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_provisioning_preferences): <p>An object that contains information about the provisioning preferences for a stack set.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::UpdateProvisionedProduct::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_tags): <p>One or more tags. Requires the product to have <code>RESOURCE_UPDATE</code> constraint with <code>TagUpdatesOnProvisionedProduct</code> set to <code>ALLOWED</code> to allow tag updates.</p>
    ///   - [`update_token(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::update_token) / [`set_update_token(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProduct::set_update_token): <p>The idempotency token that uniquely identifies the provisioning update request.</p>
                            /// - On success, responds with [`UpdateProvisionedProductOutput`](crate::output::UpdateProvisionedProductOutput) with field(s):
    ///   - [`record_detail(Option<RecordDetail>)`](crate::output::UpdateProvisionedProductOutput::record_detail): <p>Information about the result of the request.</p>
                            /// - On failure, responds with [`SdkError<UpdateProvisionedProductError>`](crate::error::UpdateProvisionedProductError)
    pub fn update_provisioned_product(&self) -> crate::client::fluent_builders::UpdateProvisionedProduct {
                                crate::client::fluent_builders::UpdateProvisionedProduct::new(self.handle.clone())
                            }
}

