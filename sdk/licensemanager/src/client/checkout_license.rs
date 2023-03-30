// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CheckoutLicense`](crate::client::fluent_builders::CheckoutLicense) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`product_sku(impl Into<String>)`](crate::client::fluent_builders::CheckoutLicense::product_sku) / [`set_product_sku(Option<String>)`](crate::client::fluent_builders::CheckoutLicense::set_product_sku): <p>Product SKU.</p>
    ///   - [`checkout_type(CheckoutType)`](crate::client::fluent_builders::CheckoutLicense::checkout_type) / [`set_checkout_type(Option<CheckoutType>)`](crate::client::fluent_builders::CheckoutLicense::set_checkout_type): <p>Checkout type.</p>
    ///   - [`key_fingerprint(impl Into<String>)`](crate::client::fluent_builders::CheckoutLicense::key_fingerprint) / [`set_key_fingerprint(Option<String>)`](crate::client::fluent_builders::CheckoutLicense::set_key_fingerprint): <p>Key fingerprint identifying the license.</p>
    ///   - [`entitlements(Vec<EntitlementData>)`](crate::client::fluent_builders::CheckoutLicense::entitlements) / [`set_entitlements(Option<Vec<EntitlementData>>)`](crate::client::fluent_builders::CheckoutLicense::set_entitlements): <p>License entitlements.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CheckoutLicense::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CheckoutLicense::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    ///   - [`beneficiary(impl Into<String>)`](crate::client::fluent_builders::CheckoutLicense::beneficiary) / [`set_beneficiary(Option<String>)`](crate::client::fluent_builders::CheckoutLicense::set_beneficiary): <p>License beneficiary.</p>
    ///   - [`node_id(impl Into<String>)`](crate::client::fluent_builders::CheckoutLicense::node_id) / [`set_node_id(Option<String>)`](crate::client::fluent_builders::CheckoutLicense::set_node_id): <p>Node ID.</p>
                            /// - On success, responds with [`CheckoutLicenseOutput`](crate::output::CheckoutLicenseOutput) with field(s):
    ///   - [`checkout_type(Option<CheckoutType>)`](crate::output::CheckoutLicenseOutput::checkout_type): <p>Checkout type.</p>
    ///   - [`license_consumption_token(Option<String>)`](crate::output::CheckoutLicenseOutput::license_consumption_token): <p>License consumption token.</p>
    ///   - [`entitlements_allowed(Option<Vec<EntitlementData>>)`](crate::output::CheckoutLicenseOutput::entitlements_allowed): <p>Allowed license entitlements.</p>
    ///   - [`signed_token(Option<String>)`](crate::output::CheckoutLicenseOutput::signed_token): <p>Signed token.</p>
    ///   - [`node_id(Option<String>)`](crate::output::CheckoutLicenseOutput::node_id): <p>Node ID.</p>
    ///   - [`issued_at(Option<String>)`](crate::output::CheckoutLicenseOutput::issued_at): <p>Date and time at which the license checkout is issued.</p>
    ///   - [`expiration(Option<String>)`](crate::output::CheckoutLicenseOutput::expiration): <p>Date and time at which the license checkout expires.</p>
    ///   - [`license_arn(Option<String>)`](crate::output::CheckoutLicenseOutput::license_arn): <p>Amazon Resource Name (ARN) of the checkout license.</p>
                            /// - On failure, responds with [`SdkError<CheckoutLicenseError>`](crate::error::CheckoutLicenseError)
    pub fn checkout_license(&self) -> crate::client::fluent_builders::CheckoutLicense {
                                crate::client::fluent_builders::CheckoutLicense::new(self.handle.clone())
                            }
}

