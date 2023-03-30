// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetBucketMetricData`](crate::client::fluent_builders::GetBucketMetricData) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket_name(impl Into<String>)`](crate::client::fluent_builders::GetBucketMetricData::bucket_name) / [`set_bucket_name(Option<String>)`](crate::client::fluent_builders::GetBucketMetricData::set_bucket_name): <p>The name of the bucket for which to get metric data.</p>
    ///   - [`metric_name(BucketMetricName)`](crate::client::fluent_builders::GetBucketMetricData::metric_name) / [`set_metric_name(Option<BucketMetricName>)`](crate::client::fluent_builders::GetBucketMetricData::set_metric_name): <p>The metric for which you want to return information.</p>  <p>Valid bucket metric names are listed below, along with the most useful statistics to include in your request, and the published unit value.</p> <note>   <p>These bucket metrics are reported once per day.</p>  </note>  <ul>   <li> <p> <b> <code>BucketSizeBytes</code> </b> - The amount of data in bytes stored in a bucket. This value is calculated by summing the size of all objects in the bucket (including object versions), including the size of all parts for all incomplete multipart uploads to the bucket.</p> <p>Statistics: The most useful statistic is <code>Maximum</code>.</p> <p>Unit: The published unit is <code>Bytes</code>.</p> </li>   <li> <p> <b> <code>NumberOfObjects</code> </b> - The total number of objects stored in a bucket. This value is calculated by counting all objects in the bucket (including object versions) and the total number of parts for all incomplete multipart uploads to the bucket.</p> <p>Statistics: The most useful statistic is <code>Average</code>.</p> <p>Unit: The published unit is <code>Count</code>.</p> </li>  </ul>
    ///   - [`start_time(DateTime)`](crate::client::fluent_builders::GetBucketMetricData::start_time) / [`set_start_time(Option<DateTime>)`](crate::client::fluent_builders::GetBucketMetricData::set_start_time): <p>The timestamp indicating the earliest data to be returned.</p>
    ///   - [`end_time(DateTime)`](crate::client::fluent_builders::GetBucketMetricData::end_time) / [`set_end_time(Option<DateTime>)`](crate::client::fluent_builders::GetBucketMetricData::set_end_time): <p>The timestamp indicating the latest data to be returned.</p>
    ///   - [`period(i32)`](crate::client::fluent_builders::GetBucketMetricData::period) / [`set_period(i32)`](crate::client::fluent_builders::GetBucketMetricData::set_period): <p>The granularity, in seconds, of the returned data points.</p> <note>   <p>Bucket storage metrics are reported once per day. Therefore, you should specify a period of 86400 seconds, which is the number of seconds in a day.</p>  </note>
    ///   - [`statistics(Vec<MetricStatistic>)`](crate::client::fluent_builders::GetBucketMetricData::statistics) / [`set_statistics(Option<Vec<MetricStatistic>>)`](crate::client::fluent_builders::GetBucketMetricData::set_statistics): <p>The statistic for the metric.</p>  <p>The following statistics are available:</p>  <ul>   <li> <p> <code>Minimum</code> - The lowest value observed during the specified period. Use this value to determine low volumes of activity for your application.</p> </li>   <li> <p> <code>Maximum</code> - The highest value observed during the specified period. Use this value to determine high volumes of activity for your application.</p> </li>   <li> <p> <code>Sum</code> - The sum of all values submitted for the matching metric. You can use this statistic to determine the total volume of a metric.</p> </li>   <li> <p> <code>Average</code> - The value of <code>Sum</code> / <code>SampleCount</code> during the specified period. By comparing this statistic with the <code>Minimum</code> and <code>Maximum</code> values, you can determine the full scope of a metric and how close the average use is to the <code>Minimum</code> and <code>Maximum</code> values. This comparison helps you to know when to increase or decrease your resources.</p> </li>   <li> <p> <code>SampleCount</code> - The count, or number, of data points used for the statistical calculation.</p> </li>  </ul>
    ///   - [`unit(MetricUnit)`](crate::client::fluent_builders::GetBucketMetricData::unit) / [`set_unit(Option<MetricUnit>)`](crate::client::fluent_builders::GetBucketMetricData::set_unit): <p>The unit for the metric data request.</p>  <p>Valid units depend on the metric data being requested. For the valid units with each available metric, see the <code>metricName</code> parameter.</p>
                            /// - On success, responds with [`GetBucketMetricDataOutput`](crate::output::GetBucketMetricDataOutput) with field(s):
    ///   - [`metric_name(Option<BucketMetricName>)`](crate::output::GetBucketMetricDataOutput::metric_name): <p>The name of the metric returned.</p>
    ///   - [`metric_data(Option<Vec<MetricDatapoint>>)`](crate::output::GetBucketMetricDataOutput::metric_data): <p>An array of objects that describe the metric data returned.</p>
                            /// - On failure, responds with [`SdkError<GetBucketMetricDataError>`](crate::error::GetBucketMetricDataError)
    pub fn get_bucket_metric_data(&self) -> crate::client::fluent_builders::GetBucketMetricData {
                                crate::client::fluent_builders::GetBucketMetricData::new(self.handle.clone())
                            }
}

