// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateProvisionedProductProperties`](crate::client::fluent_builders::UpdateProvisionedProductProperties) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accept_language(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProductProperties::accept_language) / [`set_accept_language(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProductProperties::set_accept_language): <p>The language code.</p>  <ul>   <li> <p> <code>en</code> - English (default)</p> </li>   <li> <p> <code>jp</code> - Japanese</p> </li>   <li> <p> <code>zh</code> - Chinese</p> </li>  </ul>
    ///   - [`provisioned_product_id(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProductProperties::provisioned_product_id) / [`set_provisioned_product_id(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProductProperties::set_provisioned_product_id): <p>The identifier of the provisioned product.</p>
    ///   - [`provisioned_product_properties(HashMap<PropertyKey, String>)`](crate::client::fluent_builders::UpdateProvisionedProductProperties::provisioned_product_properties) / [`set_provisioned_product_properties(Option<HashMap<PropertyKey, String>>)`](crate::client::fluent_builders::UpdateProvisionedProductProperties::set_provisioned_product_properties): <p>A map that contains the provisioned product properties to be updated.</p>  <p>The <code>LAUNCH_ROLE</code> key accepts role ARNs. This key allows an administrator to call <code>UpdateProvisionedProductProperties</code> to update the launch role that is associated with a provisioned product. This role is used when an end user calls a provisioning operation such as <code>UpdateProvisionedProduct</code>, <code>TerminateProvisionedProduct</code>, or <code>ExecuteProvisionedProductServiceAction</code>. Only a role ARN is valid. A user ARN is invalid. </p>  <p>The <code>OWNER</code> key accepts IAM user ARNs, IAM role ARNs, and STS assumed-role ARNs. The owner is the user that has permission to see, update, terminate, and execute service actions in the provisioned product.</p>  <p>The administrator can change the owner of a provisioned product to another IAM or STS entity within the same account. Both end user owners and administrators can see ownership history of the provisioned product using the <code>ListRecordHistory</code> API. The new owner can describe all past records for the provisioned product using the <code>DescribeRecord</code> API. The previous owner can no longer use <code>DescribeRecord</code>, but can still see the product's history from when he was an owner using <code>ListRecordHistory</code>.</p>  <p>If a provisioned product ownership is assigned to an end user, they can see and perform any action through the API or Service Catalog console such as update, terminate, and execute service actions. If an end user provisions a product and the owner is updated to someone else, they will no longer be able to see or perform any actions through API or the Service Catalog console on that provisioned product.</p>
    ///   - [`idempotency_token(impl Into<String>)`](crate::client::fluent_builders::UpdateProvisionedProductProperties::idempotency_token) / [`set_idempotency_token(Option<String>)`](crate::client::fluent_builders::UpdateProvisionedProductProperties::set_idempotency_token): <p>The idempotency token that uniquely identifies the provisioning product update request.</p>
                            /// - On success, responds with [`UpdateProvisionedProductPropertiesOutput`](crate::output::UpdateProvisionedProductPropertiesOutput) with field(s):
    ///   - [`provisioned_product_id(Option<String>)`](crate::output::UpdateProvisionedProductPropertiesOutput::provisioned_product_id): <p>The provisioned product identifier.</p>
    ///   - [`provisioned_product_properties(Option<HashMap<PropertyKey, String>>)`](crate::output::UpdateProvisionedProductPropertiesOutput::provisioned_product_properties): <p>A map that contains the properties updated.</p>
    ///   - [`record_id(Option<String>)`](crate::output::UpdateProvisionedProductPropertiesOutput::record_id): <p>The identifier of the record.</p>
    ///   - [`status(Option<RecordStatus>)`](crate::output::UpdateProvisionedProductPropertiesOutput::status): <p>The status of the request.</p>
                            /// - On failure, responds with [`SdkError<UpdateProvisionedProductPropertiesError>`](crate::error::UpdateProvisionedProductPropertiesError)
    pub fn update_provisioned_product_properties(&self) -> crate::client::fluent_builders::UpdateProvisionedProductProperties {
                                crate::client::fluent_builders::UpdateProvisionedProductProperties::new(self.handle.clone())
                            }
}

