// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateEventDestination`](crate::client::fluent_builders::UpdateEventDestination) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::UpdateEventDestination::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::UpdateEventDestination::set_configuration_set_name): <p>The configuration set to update with the new event destination. Valid values for this can be the ConfigurationSetName or ConfigurationSetArn.</p>
    ///   - [`event_destination_name(impl Into<String>)`](crate::client::fluent_builders::UpdateEventDestination::event_destination_name) / [`set_event_destination_name(Option<String>)`](crate::client::fluent_builders::UpdateEventDestination::set_event_destination_name): <p>The name to use for the event destination.</p>
    ///   - [`enabled(bool)`](crate::client::fluent_builders::UpdateEventDestination::enabled) / [`set_enabled(Option<bool>)`](crate::client::fluent_builders::UpdateEventDestination::set_enabled): <p>When set to true logging is enabled.</p>
    ///   - [`matching_event_types(Vec<EventType>)`](crate::client::fluent_builders::UpdateEventDestination::matching_event_types) / [`set_matching_event_types(Option<Vec<EventType>>)`](crate::client::fluent_builders::UpdateEventDestination::set_matching_event_types): <p>An array of event types that determine which events to log.</p>
    ///   - [`cloud_watch_logs_destination(CloudWatchLogsDestination)`](crate::client::fluent_builders::UpdateEventDestination::cloud_watch_logs_destination) / [`set_cloud_watch_logs_destination(Option<CloudWatchLogsDestination>)`](crate::client::fluent_builders::UpdateEventDestination::set_cloud_watch_logs_destination): <p>An object that contains information about an event destination that sends data to CloudWatch Logs.</p>
    ///   - [`kinesis_firehose_destination(KinesisFirehoseDestination)`](crate::client::fluent_builders::UpdateEventDestination::kinesis_firehose_destination) / [`set_kinesis_firehose_destination(Option<KinesisFirehoseDestination>)`](crate::client::fluent_builders::UpdateEventDestination::set_kinesis_firehose_destination): <p>An object that contains information about an event destination for logging to Kinesis Data Firehose.</p>
    ///   - [`sns_destination(SnsDestination)`](crate::client::fluent_builders::UpdateEventDestination::sns_destination) / [`set_sns_destination(Option<SnsDestination>)`](crate::client::fluent_builders::UpdateEventDestination::set_sns_destination): <p>An object that contains information about an event destination that sends data to Amazon SNS.</p>
                            /// - On success, responds with [`UpdateEventDestinationOutput`](crate::output::UpdateEventDestinationOutput) with field(s):
    ///   - [`configuration_set_arn(Option<String>)`](crate::output::UpdateEventDestinationOutput::configuration_set_arn): <p>The Amazon Resource Name (ARN) for the ConfigurationSet that was updated.</p>
    ///   - [`configuration_set_name(Option<String>)`](crate::output::UpdateEventDestinationOutput::configuration_set_name): <p>The name of the configuration set.</p>
    ///   - [`event_destination(Option<EventDestination>)`](crate::output::UpdateEventDestinationOutput::event_destination): <p>An EventDestination object containing the details of where events will be logged. </p>
                            /// - On failure, responds with [`SdkError<UpdateEventDestinationError>`](crate::error::UpdateEventDestinationError)
    pub fn update_event_destination(&self) -> crate::client::fluent_builders::UpdateEventDestination {
                                crate::client::fluent_builders::UpdateEventDestination::new(self.handle.clone())
                            }
}

