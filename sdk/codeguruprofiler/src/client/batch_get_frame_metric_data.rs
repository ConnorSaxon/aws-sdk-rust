// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchGetFrameMetricData`](crate::client::fluent_builders::BatchGetFrameMetricData) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`profiling_group_name(impl Into<String>)`](crate::client::fluent_builders::BatchGetFrameMetricData::profiling_group_name) / [`set_profiling_group_name(Option<String>)`](crate::client::fluent_builders::BatchGetFrameMetricData::set_profiling_group_name): <p> The name of the profiling group associated with the the frame metrics used to return the time series values. </p>
    ///   - [`start_time(DateTime)`](crate::client::fluent_builders::BatchGetFrameMetricData::start_time) / [`set_start_time(Option<DateTime>)`](crate::client::fluent_builders::BatchGetFrameMetricData::set_start_time): <p> The start time of the time period for the frame metrics used to return the time series values. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    ///   - [`end_time(DateTime)`](crate::client::fluent_builders::BatchGetFrameMetricData::end_time) / [`set_end_time(Option<DateTime>)`](crate::client::fluent_builders::BatchGetFrameMetricData::set_end_time): <p> The end time of the time period for the returned time series values. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    ///   - [`period(impl Into<String>)`](crate::client::fluent_builders::BatchGetFrameMetricData::period) / [`set_period(Option<String>)`](crate::client::fluent_builders::BatchGetFrameMetricData::set_period): <p> The duration of the frame metrics used to return the time series values. Specify using the ISO 8601 format. The maximum period duration is one day (<code>PT24H</code> or <code>P1D</code>). </p>
    ///   - [`target_resolution(AggregationPeriod)`](crate::client::fluent_builders::BatchGetFrameMetricData::target_resolution) / [`set_target_resolution(Option<AggregationPeriod>)`](crate::client::fluent_builders::BatchGetFrameMetricData::set_target_resolution): <p>The requested resolution of time steps for the returned time series of values. If the requested target resolution is not available due to data not being retained we provide a best effort result by falling back to the most granular available resolution after the target resolution. There are 3 valid values. </p>  <ul>   <li> <p> <code>P1D</code> — 1 day </p> </li>   <li> <p> <code>PT1H</code> — 1 hour </p> </li>   <li> <p> <code>PT5M</code> — 5 minutes </p> </li>  </ul>
    ///   - [`frame_metrics(Vec<FrameMetric>)`](crate::client::fluent_builders::BatchGetFrameMetricData::frame_metrics) / [`set_frame_metrics(Option<Vec<FrameMetric>>)`](crate::client::fluent_builders::BatchGetFrameMetricData::set_frame_metrics): <p> The details of the metrics that are used to request a time series of values. The metric includes the name of the frame, the aggregation type to calculate the metric value for the frame, and the thread states to use to get the count for the metric value of the frame.</p>
                            /// - On success, responds with [`BatchGetFrameMetricDataOutput`](crate::output::BatchGetFrameMetricDataOutput) with field(s):
    ///   - [`start_time(Option<DateTime>)`](crate::output::BatchGetFrameMetricDataOutput::start_time): <p> The start time of the time period for the returned time series values. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    ///   - [`end_time(Option<DateTime>)`](crate::output::BatchGetFrameMetricDataOutput::end_time): <p> The end time of the time period for the returned time series values. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    ///   - [`resolution(Option<AggregationPeriod>)`](crate::output::BatchGetFrameMetricDataOutput::resolution): <p>Resolution or granularity of the profile data used to generate the time series. This is the value used to jump through time steps in a time series. There are 3 valid values. </p>  <ul>   <li> <p> <code>P1D</code> — 1 day </p> </li>   <li> <p> <code>PT1H</code> — 1 hour </p> </li>   <li> <p> <code>PT5M</code> — 5 minutes </p> </li>  </ul>
    ///   - [`end_times(Option<Vec<TimestampStructure>>)`](crate::output::BatchGetFrameMetricDataOutput::end_times): <p> List of instances, or time steps, in the time series. For example, if the <code>period</code> is one day (<code>PT24H)</code>), and the <code>resolution</code> is five minutes (<code>PT5M</code>), then there are 288 <code>endTimes</code> in the list that are each five minutes appart. </p>
    ///   - [`unprocessed_end_times(Option<HashMap<String, Vec<TimestampStructure>>>)`](crate::output::BatchGetFrameMetricDataOutput::unprocessed_end_times): <p>List of instances which remained unprocessed. This will create a missing time step in the list of end times.</p>
    ///   - [`frame_metric_data(Option<Vec<FrameMetricDatum>>)`](crate::output::BatchGetFrameMetricDataOutput::frame_metric_data): <p>Details of the metrics to request a time series of values. The metric includes the name of the frame, the aggregation type to calculate the metric value for the frame, and the thread states to use to get the count for the metric value of the frame.</p>
                            /// - On failure, responds with [`SdkError<BatchGetFrameMetricDataError>`](crate::error::BatchGetFrameMetricDataError)
    pub fn batch_get_frame_metric_data(&self) -> crate::client::fluent_builders::BatchGetFrameMetricData {
                                crate::client::fluent_builders::BatchGetFrameMetricData::new(self.handle.clone())
                            }
}

