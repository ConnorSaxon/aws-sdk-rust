// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteMetricAttribution`](crate::client::fluent_builders::DeleteMetricAttribution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`metric_attribution_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteMetricAttribution::metric_attribution_arn) / [`set_metric_attribution_arn(Option<String>)`](crate::client::fluent_builders::DeleteMetricAttribution::set_metric_attribution_arn): <p>The metric attribution's Amazon Resource Name (ARN).</p>
                            /// - On success, responds with [`DeleteMetricAttributionOutput`](crate::output::DeleteMetricAttributionOutput)
                            /// - On failure, responds with [`SdkError<DeleteMetricAttributionError>`](crate::error::DeleteMetricAttributionError)
    pub fn delete_metric_attribution(&self) -> crate::client::fluent_builders::DeleteMetricAttribution {
                                crate::client::fluent_builders::DeleteMetricAttribution::new(self.handle.clone())
                            }
}

