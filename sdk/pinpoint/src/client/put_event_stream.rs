// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutEventStream`](crate::client::fluent_builders::PutEventStream) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::PutEventStream::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::PutEventStream::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`write_event_stream(WriteEventStream)`](crate::client::fluent_builders::PutEventStream::write_event_stream) / [`set_write_event_stream(Option<WriteEventStream>)`](crate::client::fluent_builders::PutEventStream::set_write_event_stream): <p>Specifies the Amazon Resource Name (ARN) of an event stream to publish events to and the AWS Identity and Access Management (IAM) role to use when publishing those events.</p>
                            /// - On success, responds with [`PutEventStreamOutput`](crate::output::PutEventStreamOutput) with field(s):
    ///   - [`event_stream(Option<EventStream>)`](crate::output::PutEventStreamOutput::event_stream): <p>Specifies settings for publishing event data to an Amazon Kinesis data stream or an Amazon Kinesis Data Firehose delivery stream.</p>
                            /// - On failure, responds with [`SdkError<PutEventStreamError>`](crate::error::PutEventStreamError)
    pub fn put_event_stream(&self) -> crate::client::fluent_builders::PutEventStream {
                                crate::client::fluent_builders::PutEventStream::new(self.handle.clone())
                            }
}

