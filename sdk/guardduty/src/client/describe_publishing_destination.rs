// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribePublishingDestination`](crate::client::fluent_builders::DescribePublishingDestination) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`detector_id(impl Into<String>)`](crate::client::fluent_builders::DescribePublishingDestination::detector_id) / [`set_detector_id(Option<String>)`](crate::client::fluent_builders::DescribePublishingDestination::set_detector_id): <p>The unique ID of the detector associated with the publishing destination to retrieve.</p>
    ///   - [`destination_id(impl Into<String>)`](crate::client::fluent_builders::DescribePublishingDestination::destination_id) / [`set_destination_id(Option<String>)`](crate::client::fluent_builders::DescribePublishingDestination::set_destination_id): <p>The ID of the publishing destination to retrieve.</p>
                            /// - On success, responds with [`DescribePublishingDestinationOutput`](crate::output::DescribePublishingDestinationOutput) with field(s):
    ///   - [`destination_id(Option<String>)`](crate::output::DescribePublishingDestinationOutput::destination_id): <p>The ID of the publishing destination.</p>
    ///   - [`destination_type(Option<DestinationType>)`](crate::output::DescribePublishingDestinationOutput::destination_type): <p>The type of publishing destination. Currently, only Amazon S3 buckets are supported.</p>
    ///   - [`status(Option<PublishingStatus>)`](crate::output::DescribePublishingDestinationOutput::status): <p>The status of the publishing destination.</p>
    ///   - [`publishing_failure_start_timestamp(i64)`](crate::output::DescribePublishingDestinationOutput::publishing_failure_start_timestamp): <p>The time, in epoch millisecond format, at which GuardDuty was first unable to publish findings to the destination.</p>
    ///   - [`destination_properties(Option<DestinationProperties>)`](crate::output::DescribePublishingDestinationOutput::destination_properties): <p>A <code>DestinationProperties</code> object that includes the <code>DestinationArn</code> and <code>KmsKeyArn</code> of the publishing destination.</p>
                            /// - On failure, responds with [`SdkError<DescribePublishingDestinationError>`](crate::error::DescribePublishingDestinationError)
    pub fn describe_publishing_destination(&self) -> crate::client::fluent_builders::DescribePublishingDestination {
                                crate::client::fluent_builders::DescribePublishingDestination::new(self.handle.clone())
                            }
}

