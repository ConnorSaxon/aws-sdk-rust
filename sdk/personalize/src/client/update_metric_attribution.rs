// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateMetricAttribution`](crate::client::fluent_builders::UpdateMetricAttribution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`add_metrics(Vec<MetricAttribute>)`](crate::client::fluent_builders::UpdateMetricAttribution::add_metrics) / [`set_add_metrics(Option<Vec<MetricAttribute>>)`](crate::client::fluent_builders::UpdateMetricAttribution::set_add_metrics): <p>Add new metric attributes to the metric attribution.</p>
    ///   - [`remove_metrics(Vec<String>)`](crate::client::fluent_builders::UpdateMetricAttribution::remove_metrics) / [`set_remove_metrics(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateMetricAttribution::set_remove_metrics): <p>Remove metric attributes from the metric attribution.</p>
    ///   - [`metrics_output_config(MetricAttributionOutput)`](crate::client::fluent_builders::UpdateMetricAttribution::metrics_output_config) / [`set_metrics_output_config(Option<MetricAttributionOutput>)`](crate::client::fluent_builders::UpdateMetricAttribution::set_metrics_output_config): <p>An output config for the metric attribution.</p>
    ///   - [`metric_attribution_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateMetricAttribution::metric_attribution_arn) / [`set_metric_attribution_arn(Option<String>)`](crate::client::fluent_builders::UpdateMetricAttribution::set_metric_attribution_arn): <p>The Amazon Resource Name (ARN) for the metric attribution to update.</p>
                            /// - On success, responds with [`UpdateMetricAttributionOutput`](crate::output::UpdateMetricAttributionOutput) with field(s):
    ///   - [`metric_attribution_arn(Option<String>)`](crate::output::UpdateMetricAttributionOutput::metric_attribution_arn): <p>The Amazon Resource Name (ARN) for the metric attribution that you updated.</p>
                            /// - On failure, responds with [`SdkError<UpdateMetricAttributionError>`](crate::error::UpdateMetricAttributionError)
    pub fn update_metric_attribution(&self) -> crate::client::fluent_builders::UpdateMetricAttribution {
                                crate::client::fluent_builders::UpdateMetricAttribution::new(self.handle.clone())
                            }
}

