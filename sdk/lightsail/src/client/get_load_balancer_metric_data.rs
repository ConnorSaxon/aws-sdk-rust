// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetLoadBalancerMetricData`](crate::client::fluent_builders::GetLoadBalancerMetricData) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`load_balancer_name(impl Into<String>)`](crate::client::fluent_builders::GetLoadBalancerMetricData::load_balancer_name) / [`set_load_balancer_name(Option<String>)`](crate::client::fluent_builders::GetLoadBalancerMetricData::set_load_balancer_name): <p>The name of the load balancer.</p>
    ///   - [`metric_name(LoadBalancerMetricName)`](crate::client::fluent_builders::GetLoadBalancerMetricData::metric_name) / [`set_metric_name(Option<LoadBalancerMetricName>)`](crate::client::fluent_builders::GetLoadBalancerMetricData::set_metric_name): <p>The metric for which you want to return information.</p>  <p>Valid load balancer metric names are listed below, along with the most useful <code>statistics</code> to include in your request, and the published <code>unit</code> value.</p>  <ul>   <li> <p> <b> <code>ClientTLSNegotiationErrorCount</code> </b> - The number of TLS connections initiated by the client that did not establish a session with the load balancer due to a TLS error generated by the load balancer. Possible causes include a mismatch of ciphers or protocols.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>   <li> <p> <b> <code>HealthyHostCount</code> </b> - The number of target instances that are considered healthy.</p> <p> <code>Statistics</code>: The most useful statistic are <code>Average</code>, <code>Minimum</code>, and <code>Maximum</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>   <li> <p> <b> <code>HTTPCode_Instance_2XX_Count</code> </b> - The number of HTTP 2XX response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>   <li> <p> <b> <code>HTTPCode_Instance_3XX_Count</code> </b> - The number of HTTP 3XX response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>   <li> <p> <b> <code>HTTPCode_Instance_4XX_Count</code> </b> - The number of HTTP 4XX response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>   <li> <p> <b> <code>HTTPCode_Instance_5XX_Count</code> </b> - The number of HTTP 5XX response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>   <li> <p> <b> <code>HTTPCode_LB_4XX_Count</code> </b> - The number of HTTP 4XX client error codes that originated from the load balancer. Client errors are generated when requests are malformed or incomplete. These requests were not received by the target instance. This count does not include response codes generated by the target instances.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>   <li> <p> <b> <code>HTTPCode_LB_5XX_Count</code> </b> - The number of HTTP 5XX server error codes that originated from the load balancer. This does not include any response codes generated by the target instance. This metric is reported if there are no healthy instances attached to the load balancer, or if the request rate exceeds the capacity of the instances (spillover) or the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>   <li> <p> <b> <code>InstanceResponseTime</code> </b> - The time elapsed, in seconds, after the request leaves the load balancer until a response from the target instance is received.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> <p> <code>Unit</code>: The published unit is <code>Seconds</code>.</p> </li>   <li> <p> <b> <code>RejectedConnectionCount</code> </b> - The number of connections that were rejected because the load balancer had reached its maximum number of connections.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>   <li> <p> <b> <code>RequestCount</code> </b> - The number of requests processed over IPv4. This count includes only the requests with a response generated by a target instance of the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>   <li> <p> <b> <code>UnhealthyHostCount</code> </b> - The number of target instances that are considered unhealthy.</p> <p> <code>Statistics</code>: The most useful statistic are <code>Average</code>, <code>Minimum</code>, and <code>Maximum</code>.</p> <p> <code>Unit</code>: The published unit is <code>Count</code>.</p> </li>  </ul>
    ///   - [`period(i32)`](crate::client::fluent_builders::GetLoadBalancerMetricData::period) / [`set_period(i32)`](crate::client::fluent_builders::GetLoadBalancerMetricData::set_period): <p>The granularity, in seconds, of the returned data points.</p>
    ///   - [`start_time(DateTime)`](crate::client::fluent_builders::GetLoadBalancerMetricData::start_time) / [`set_start_time(Option<DateTime>)`](crate::client::fluent_builders::GetLoadBalancerMetricData::set_start_time): <p>The start time of the period.</p>
    ///   - [`end_time(DateTime)`](crate::client::fluent_builders::GetLoadBalancerMetricData::end_time) / [`set_end_time(Option<DateTime>)`](crate::client::fluent_builders::GetLoadBalancerMetricData::set_end_time): <p>The end time of the period.</p>
    ///   - [`unit(MetricUnit)`](crate::client::fluent_builders::GetLoadBalancerMetricData::unit) / [`set_unit(Option<MetricUnit>)`](crate::client::fluent_builders::GetLoadBalancerMetricData::set_unit): <p>The unit for the metric data request. Valid units depend on the metric data being requested. For the valid units with each available metric, see the <code>metricName</code> parameter.</p>
    ///   - [`statistics(Vec<MetricStatistic>)`](crate::client::fluent_builders::GetLoadBalancerMetricData::statistics) / [`set_statistics(Option<Vec<MetricStatistic>>)`](crate::client::fluent_builders::GetLoadBalancerMetricData::set_statistics): <p>The statistic for the metric.</p>  <p>The following statistics are available:</p>  <ul>   <li> <p> <code>Minimum</code> - The lowest value observed during the specified period. Use this value to determine low volumes of activity for your application.</p> </li>   <li> <p> <code>Maximum</code> - The highest value observed during the specified period. Use this value to determine high volumes of activity for your application.</p> </li>   <li> <p> <code>Sum</code> - All values submitted for the matching metric added together. You can use this statistic to determine the total volume of a metric.</p> </li>   <li> <p> <code>Average</code> - The value of Sum / SampleCount during the specified period. By comparing this statistic with the Minimum and Maximum values, you can determine the full scope of a metric and how close the average use is to the Minimum and Maximum values. This comparison helps you to know when to increase or decrease your resources.</p> </li>   <li> <p> <code>SampleCount</code> - The count, or number, of data points used for the statistical calculation.</p> </li>  </ul>
                            /// - On success, responds with [`GetLoadBalancerMetricDataOutput`](crate::output::GetLoadBalancerMetricDataOutput) with field(s):
    ///   - [`metric_name(Option<LoadBalancerMetricName>)`](crate::output::GetLoadBalancerMetricDataOutput::metric_name): <p>The name of the metric returned.</p>
    ///   - [`metric_data(Option<Vec<MetricDatapoint>>)`](crate::output::GetLoadBalancerMetricDataOutput::metric_data): <p>An array of objects that describe the metric data returned.</p>
                            /// - On failure, responds with [`SdkError<GetLoadBalancerMetricDataError>`](crate::error::GetLoadBalancerMetricDataError)
    pub fn get_load_balancer_metric_data(&self) -> crate::client::fluent_builders::GetLoadBalancerMetricData {
                                crate::client::fluent_builders::GetLoadBalancerMetricData::new(self.handle.clone())
                            }
}

