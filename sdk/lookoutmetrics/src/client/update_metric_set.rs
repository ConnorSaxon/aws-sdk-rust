// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateMetricSet`](crate::client::fluent_builders::UpdateMetricSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`metric_set_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateMetricSet::metric_set_arn) / [`set_metric_set_arn(Option<String>)`](crate::client::fluent_builders::UpdateMetricSet::set_metric_set_arn): <p>The ARN of the dataset to update.</p>
    ///   - [`metric_set_description(impl Into<String>)`](crate::client::fluent_builders::UpdateMetricSet::metric_set_description) / [`set_metric_set_description(Option<String>)`](crate::client::fluent_builders::UpdateMetricSet::set_metric_set_description): <p>The dataset's description.</p>
    ///   - [`metric_list(Vec<Metric>)`](crate::client::fluent_builders::UpdateMetricSet::metric_list) / [`set_metric_list(Option<Vec<Metric>>)`](crate::client::fluent_builders::UpdateMetricSet::set_metric_list): <p>The metric list.</p>
    ///   - [`offset(i32)`](crate::client::fluent_builders::UpdateMetricSet::offset) / [`set_offset(Option<i32>)`](crate::client::fluent_builders::UpdateMetricSet::set_offset): <p>After an interval ends, the amount of seconds that the detector waits before importing data. Offset is only supported for S3, Redshift, Athena and datasources.</p>
    ///   - [`timestamp_column(TimestampColumn)`](crate::client::fluent_builders::UpdateMetricSet::timestamp_column) / [`set_timestamp_column(Option<TimestampColumn>)`](crate::client::fluent_builders::UpdateMetricSet::set_timestamp_column): <p>The timestamp column.</p>
    ///   - [`dimension_list(Vec<String>)`](crate::client::fluent_builders::UpdateMetricSet::dimension_list) / [`set_dimension_list(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateMetricSet::set_dimension_list): <p>The dimension list.</p>
    ///   - [`metric_set_frequency(Frequency)`](crate::client::fluent_builders::UpdateMetricSet::metric_set_frequency) / [`set_metric_set_frequency(Option<Frequency>)`](crate::client::fluent_builders::UpdateMetricSet::set_metric_set_frequency): <p>The dataset's interval.</p>
    ///   - [`metric_source(MetricSource)`](crate::client::fluent_builders::UpdateMetricSet::metric_source) / [`set_metric_source(Option<MetricSource>)`](crate::client::fluent_builders::UpdateMetricSet::set_metric_source): <p>Contains information about source data used to generate metrics.</p>
    ///   - [`dimension_filter_list(Vec<MetricSetDimensionFilter>)`](crate::client::fluent_builders::UpdateMetricSet::dimension_filter_list) / [`set_dimension_filter_list(Option<Vec<MetricSetDimensionFilter>>)`](crate::client::fluent_builders::UpdateMetricSet::set_dimension_filter_list): <p>Describes a list of filters for choosing specific dimensions and specific values. Each filter consists of the dimension and one of its values that you want to include. When multiple dimensions or values are specified, the dimensions are joined with an AND operation and the values are joined with an OR operation.</p>
                            /// - On success, responds with [`UpdateMetricSetOutput`](crate::output::UpdateMetricSetOutput) with field(s):
    ///   - [`metric_set_arn(Option<String>)`](crate::output::UpdateMetricSetOutput::metric_set_arn): <p>The ARN of the dataset.</p>
                            /// - On failure, responds with [`SdkError<UpdateMetricSetError>`](crate::error::UpdateMetricSetError)
    pub fn update_metric_set(&self) -> crate::client::fluent_builders::UpdateMetricSet {
                                crate::client::fluent_builders::UpdateMetricSet::new(self.handle.clone())
                            }
}

