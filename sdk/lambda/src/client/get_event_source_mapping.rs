// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetEventSourceMapping`](crate::client::fluent_builders::GetEventSourceMapping) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`uuid(impl Into<String>)`](crate::client::fluent_builders::GetEventSourceMapping::uuid) / [`set_uuid(Option<String>)`](crate::client::fluent_builders::GetEventSourceMapping::set_uuid): <p>The identifier of the event source mapping.</p>
                            /// - On success, responds with [`GetEventSourceMappingOutput`](crate::output::GetEventSourceMappingOutput) with field(s):
    ///   - [`uuid(Option<String>)`](crate::output::GetEventSourceMappingOutput::uuid): <p>The identifier of the event source mapping.</p>
    ///   - [`starting_position(Option<EventSourcePosition>)`](crate::output::GetEventSourceMappingOutput::starting_position): <p>The position in a stream from which to start reading. Required for Amazon Kinesis, Amazon DynamoDB, and Amazon MSK stream sources. <code>AT_TIMESTAMP</code> is supported only for Amazon Kinesis streams.</p>
    ///   - [`starting_position_timestamp(Option<DateTime>)`](crate::output::GetEventSourceMappingOutput::starting_position_timestamp): <p>With <code>StartingPosition</code> set to <code>AT_TIMESTAMP</code>, the time from which to start reading.</p>
    ///   - [`batch_size(Option<i32>)`](crate::output::GetEventSourceMappingOutput::batch_size): <p>The maximum number of records in each batch that Lambda pulls from your stream or queue and sends to your function. Lambda passes all of the records in the batch to the function in a single call, up to the payload limit for synchronous invocation (6 MB).</p>  <p>Default value: Varies by service. For Amazon SQS, the default is 10. For all other services, the default is 100.</p>  <p>Related setting: When you set <code>BatchSize</code> to a value greater than 10, you must set <code>MaximumBatchingWindowInSeconds</code> to at least 1.</p>
    ///   - [`maximum_batching_window_in_seconds(Option<i32>)`](crate::output::GetEventSourceMappingOutput::maximum_batching_window_in_seconds): <p>The maximum amount of time, in seconds, that Lambda spends gathering records before invoking the function. You can configure <code>MaximumBatchingWindowInSeconds</code> to any value from 0 seconds to 300 seconds in increments of seconds.</p>  <p>For streams and Amazon SQS event sources, the default batching window is 0 seconds. For Amazon MSK, Self-managed Apache Kafka, and Amazon MQ event sources, the default batching window is 500 ms. Note that because you can only change <code>MaximumBatchingWindowInSeconds</code> in increments of seconds, you cannot revert back to the 500 ms default batching window after you have changed it. To restore the default batching window, you must create a new event source mapping.</p>  <p>Related setting: For streams and Amazon SQS event sources, when you set <code>BatchSize</code> to a value greater than 10, you must set <code>MaximumBatchingWindowInSeconds</code> to at least 1.</p>
    ///   - [`parallelization_factor(Option<i32>)`](crate::output::GetEventSourceMappingOutput::parallelization_factor): <p>(Streams only) The number of batches to process concurrently from each shard. The default value is 1.</p>
    ///   - [`event_source_arn(Option<String>)`](crate::output::GetEventSourceMappingOutput::event_source_arn): <p>The Amazon Resource Name (ARN) of the event source.</p>
    ///   - [`filter_criteria(Option<FilterCriteria>)`](crate::output::GetEventSourceMappingOutput::filter_criteria): <p>An object that defines the filter criteria that determine whether Lambda should process an event. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html">Lambda event filtering</a>.</p>
    ///   - [`function_arn(Option<String>)`](crate::output::GetEventSourceMappingOutput::function_arn): <p>The ARN of the Lambda function.</p>
    ///   - [`last_modified(Option<DateTime>)`](crate::output::GetEventSourceMappingOutput::last_modified): <p>The date that the event source mapping was last updated or that its state changed.</p>
    ///   - [`last_processing_result(Option<String>)`](crate::output::GetEventSourceMappingOutput::last_processing_result): <p>The result of the last Lambda invocation of your function.</p>
    ///   - [`state(Option<String>)`](crate::output::GetEventSourceMappingOutput::state): <p>The state of the event source mapping. It can be one of the following: <code>Creating</code>, <code>Enabling</code>, <code>Enabled</code>, <code>Disabling</code>, <code>Disabled</code>, <code>Updating</code>, or <code>Deleting</code>.</p>
    ///   - [`state_transition_reason(Option<String>)`](crate::output::GetEventSourceMappingOutput::state_transition_reason): <p>Indicates whether a user or Lambda made the last change to the event source mapping.</p>
    ///   - [`destination_config(Option<DestinationConfig>)`](crate::output::GetEventSourceMappingOutput::destination_config): <p>(Streams only) An Amazon SQS queue or Amazon SNS topic destination for discarded records.</p>
    ///   - [`topics(Option<Vec<String>>)`](crate::output::GetEventSourceMappingOutput::topics): <p>The name of the Kafka topic.</p>
    ///   - [`queues(Option<Vec<String>>)`](crate::output::GetEventSourceMappingOutput::queues): <p> (Amazon MQ) The name of the Amazon MQ broker destination queue to consume.</p>
    ///   - [`source_access_configurations(Option<Vec<SourceAccessConfiguration>>)`](crate::output::GetEventSourceMappingOutput::source_access_configurations): <p>An array of the authentication protocol, VPC components, or virtual host to secure and define your event source.</p>
    ///   - [`self_managed_event_source(Option<SelfManagedEventSource>)`](crate::output::GetEventSourceMappingOutput::self_managed_event_source): <p>The self-managed Apache Kafka cluster for your event source.</p>
    ///   - [`maximum_record_age_in_seconds(Option<i32>)`](crate::output::GetEventSourceMappingOutput::maximum_record_age_in_seconds): <p>(Streams only) Discard records older than the specified age. The default value is -1, which sets the maximum age to infinite. When the value is set to infinite, Lambda never discards old records. </p>
    ///   - [`bisect_batch_on_function_error(Option<bool>)`](crate::output::GetEventSourceMappingOutput::bisect_batch_on_function_error): <p>(Streams only) If the function returns an error, split the batch in two and retry. The default value is false.</p>
    ///   - [`maximum_retry_attempts(Option<i32>)`](crate::output::GetEventSourceMappingOutput::maximum_retry_attempts): <p>(Streams only) Discard records after the specified number of retries. The default value is -1, which sets the maximum number of retries to infinite. When MaximumRetryAttempts is infinite, Lambda retries failed records until the record expires in the event source.</p>
    ///   - [`tumbling_window_in_seconds(Option<i32>)`](crate::output::GetEventSourceMappingOutput::tumbling_window_in_seconds): <p>(Streams only) The duration in seconds of a processing window. The range is 1–900 seconds.</p>
    ///   - [`function_response_types(Option<Vec<FunctionResponseType>>)`](crate::output::GetEventSourceMappingOutput::function_response_types): <p>(Streams and Amazon SQS) A list of current response type enums applied to the event source mapping.</p>
    ///   - [`amazon_managed_kafka_event_source_config(Option<AmazonManagedKafkaEventSourceConfig>)`](crate::output::GetEventSourceMappingOutput::amazon_managed_kafka_event_source_config): <p>Specific configuration settings for an Amazon Managed Streaming for Apache Kafka (Amazon MSK) event source.</p>
    ///   - [`self_managed_kafka_event_source_config(Option<SelfManagedKafkaEventSourceConfig>)`](crate::output::GetEventSourceMappingOutput::self_managed_kafka_event_source_config): <p>Specific configuration settings for a self-managed Apache Kafka event source.</p>
    ///   - [`scaling_config(Option<ScalingConfig>)`](crate::output::GetEventSourceMappingOutput::scaling_config): <p>(Amazon SQS only) The scaling configuration for the event source. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html#events-sqs-max-concurrency">Configuring maximum concurrency for Amazon SQS event sources</a>.</p>
                            /// - On failure, responds with [`SdkError<GetEventSourceMappingError>`](crate::error::GetEventSourceMappingError)
    pub fn get_event_source_mapping(&self) -> crate::client::fluent_builders::GetEventSourceMapping {
                                crate::client::fluent_builders::GetEventSourceMapping::new(self.handle.clone())
                            }
}

