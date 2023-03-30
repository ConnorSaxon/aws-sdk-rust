// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateLongTermPricing`](crate::client::fluent_builders::CreateLongTermPricing) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`long_term_pricing_type(LongTermPricingType)`](crate::client::fluent_builders::CreateLongTermPricing::long_term_pricing_type) / [`set_long_term_pricing_type(Option<LongTermPricingType>)`](crate::client::fluent_builders::CreateLongTermPricing::set_long_term_pricing_type): <p>The type of long-term pricing option you want for the device, either 1-year or 3-year long-term pricing.</p>
    ///   - [`is_long_term_pricing_auto_renew(bool)`](crate::client::fluent_builders::CreateLongTermPricing::is_long_term_pricing_auto_renew) / [`set_is_long_term_pricing_auto_renew(Option<bool>)`](crate::client::fluent_builders::CreateLongTermPricing::set_is_long_term_pricing_auto_renew): <p>snowballty</p>  <p>Specifies whether the current long-term pricing type for the device should be renewed.</p>
    ///   - [`snowball_type(SnowballType)`](crate::client::fluent_builders::CreateLongTermPricing::snowball_type) / [`set_snowball_type(Option<SnowballType>)`](crate::client::fluent_builders::CreateLongTermPricing::set_snowball_type): <p>The type of Snow Family devices to use for the long-term pricing job.</p>
                            /// - On success, responds with [`CreateLongTermPricingOutput`](crate::output::CreateLongTermPricingOutput) with field(s):
    ///   - [`long_term_pricing_id(Option<String>)`](crate::output::CreateLongTermPricingOutput::long_term_pricing_id): <p>The ID of the long-term pricing type for the device.</p>
                            /// - On failure, responds with [`SdkError<CreateLongTermPricingError>`](crate::error::CreateLongTermPricingError)
    pub fn create_long_term_pricing(&self) -> crate::client::fluent_builders::CreateLongTermPricing {
                                crate::client::fluent_builders::CreateLongTermPricing::new(self.handle.clone())
                            }
}

