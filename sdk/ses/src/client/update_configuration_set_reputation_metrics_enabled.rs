// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateConfigurationSetReputationMetricsEnabled`](crate::client::fluent_builders::UpdateConfigurationSetReputationMetricsEnabled) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::UpdateConfigurationSetReputationMetricsEnabled::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::UpdateConfigurationSetReputationMetricsEnabled::set_configuration_set_name): <p>The name of the configuration set that you want to update.</p>
    ///   - [`enabled(bool)`](crate::client::fluent_builders::UpdateConfigurationSetReputationMetricsEnabled::enabled) / [`set_enabled(bool)`](crate::client::fluent_builders::UpdateConfigurationSetReputationMetricsEnabled::set_enabled): <p>Describes whether or not Amazon SES will publish reputation metrics for the configuration set, such as bounce and complaint rates, to Amazon CloudWatch.</p>
                            /// - On success, responds with [`UpdateConfigurationSetReputationMetricsEnabledOutput`](crate::output::UpdateConfigurationSetReputationMetricsEnabledOutput)
                            /// - On failure, responds with [`SdkError<UpdateConfigurationSetReputationMetricsEnabledError>`](crate::error::UpdateConfigurationSetReputationMetricsEnabledError)
    pub fn update_configuration_set_reputation_metrics_enabled(&self) -> crate::client::fluent_builders::UpdateConfigurationSetReputationMetricsEnabled {
                                crate::client::fluent_builders::UpdateConfigurationSetReputationMetricsEnabled::new(self.handle.clone())
                            }
}

