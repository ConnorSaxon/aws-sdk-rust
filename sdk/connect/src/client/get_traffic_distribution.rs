// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetTrafficDistribution`](crate::client::fluent_builders::GetTrafficDistribution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetTrafficDistribution::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetTrafficDistribution::set_id): <p>The identifier of the traffic distribution group.</p>
                            /// - On success, responds with [`GetTrafficDistributionOutput`](crate::output::GetTrafficDistributionOutput) with field(s):
    ///   - [`telephony_config(Option<TelephonyConfig>)`](crate::output::GetTrafficDistributionOutput::telephony_config): <p>The distribution of traffic between the instance and its replicas.</p>
    ///   - [`id(Option<String>)`](crate::output::GetTrafficDistributionOutput::id): <p>The identifier of the traffic distribution group. This can be the ID or the ARN if the API is being called in the Region where the traffic distribution group was created. The ARN must be provided if the call is from the replicated Region.</p>
    ///   - [`arn(Option<String>)`](crate::output::GetTrafficDistributionOutput::arn): <p>The Amazon Resource Name (ARN) of the traffic distribution group.</p>
                            /// - On failure, responds with [`SdkError<GetTrafficDistributionError>`](crate::error::GetTrafficDistributionError)
    pub fn get_traffic_distribution(&self) -> crate::client::fluent_builders::GetTrafficDistribution {
                                crate::client::fluent_builders::GetTrafficDistribution::new(self.handle.clone())
                            }
}

