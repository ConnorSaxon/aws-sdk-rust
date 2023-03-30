// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeMetricAttribution`](crate::client::fluent_builders::DescribeMetricAttribution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`metric_attribution_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeMetricAttribution::metric_attribution_arn) / [`set_metric_attribution_arn(Option<String>)`](crate::client::fluent_builders::DescribeMetricAttribution::set_metric_attribution_arn): <p>The metric attribution's Amazon Resource Name (ARN).</p>
                            /// - On success, responds with [`DescribeMetricAttributionOutput`](crate::output::DescribeMetricAttributionOutput) with field(s):
    ///   - [`metric_attribution(Option<MetricAttribution>)`](crate::output::DescribeMetricAttributionOutput::metric_attribution): <p>The details of the metric attribution.</p>
                            /// - On failure, responds with [`SdkError<DescribeMetricAttributionError>`](crate::error::DescribeMetricAttributionError)
    pub fn describe_metric_attribution(&self) -> crate::client::fluent_builders::DescribeMetricAttribution {
                                crate::client::fluent_builders::DescribeMetricAttribution::new(self.handle.clone())
                            }
}

