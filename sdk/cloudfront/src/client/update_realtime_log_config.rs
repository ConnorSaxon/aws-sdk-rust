// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateRealtimeLogConfig`](crate::client::fluent_builders::UpdateRealtimeLogConfig) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`end_points(Vec<EndPoint>)`](crate::client::fluent_builders::UpdateRealtimeLogConfig::end_points) / [`set_end_points(Option<Vec<EndPoint>>)`](crate::client::fluent_builders::UpdateRealtimeLogConfig::set_end_points): <p>Contains information about the Amazon Kinesis data stream where you are sending real-time log data.</p>
    ///   - [`fields(Vec<String>)`](crate::client::fluent_builders::UpdateRealtimeLogConfig::fields) / [`set_fields(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateRealtimeLogConfig::set_fields): <p>A list of fields to include in each real-time log record.</p>  <p>For more information about fields, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields">Real-time log configuration fields</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateRealtimeLogConfig::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateRealtimeLogConfig::set_name): <p>The name for this real-time log configuration.</p>
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::UpdateRealtimeLogConfig::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::UpdateRealtimeLogConfig::set_arn): <p>The Amazon Resource Name (ARN) for this real-time log configuration.</p>
    ///   - [`sampling_rate(i64)`](crate::client::fluent_builders::UpdateRealtimeLogConfig::sampling_rate) / [`set_sampling_rate(Option<i64>)`](crate::client::fluent_builders::UpdateRealtimeLogConfig::set_sampling_rate): <p>The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. You must provide an integer between 1 and 100, inclusive.</p>
                            /// - On success, responds with [`UpdateRealtimeLogConfigOutput`](crate::output::UpdateRealtimeLogConfigOutput) with field(s):
    ///   - [`realtime_log_config(Option<RealtimeLogConfig>)`](crate::output::UpdateRealtimeLogConfigOutput::realtime_log_config): <p>A real-time log configuration.</p>
                            /// - On failure, responds with [`SdkError<UpdateRealtimeLogConfigError>`](crate::error::UpdateRealtimeLogConfigError)
    pub fn update_realtime_log_config(&self) -> crate::client::fluent_builders::UpdateRealtimeLogConfig {
                                crate::client::fluent_builders::UpdateRealtimeLogConfig::new(self.handle.clone())
                            }
}

