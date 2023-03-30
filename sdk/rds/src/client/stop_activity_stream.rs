// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StopActivityStream`](crate::client::fluent_builders::StopActivityStream) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::StopActivityStream::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::StopActivityStream::set_resource_arn): <p>The Amazon Resource Name (ARN) of the DB cluster for the database activity stream. For example, <code>arn:aws:rds:us-east-1:12345667890:cluster:das-cluster</code>.</p>
    ///   - [`apply_immediately(bool)`](crate::client::fluent_builders::StopActivityStream::apply_immediately) / [`set_apply_immediately(Option<bool>)`](crate::client::fluent_builders::StopActivityStream::set_apply_immediately): <p>Specifies whether or not the database activity stream is to stop as soon as possible, regardless of the maintenance window for the database.</p>
                            /// - On success, responds with [`StopActivityStreamOutput`](crate::output::StopActivityStreamOutput) with field(s):
    ///   - [`kms_key_id(Option<String>)`](crate::output::StopActivityStreamOutput::kms_key_id): <p>The Amazon Web Services KMS key identifier used for encrypting messages in the database activity stream.</p>  <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.</p>
    ///   - [`kinesis_stream_name(Option<String>)`](crate::output::StopActivityStreamOutput::kinesis_stream_name): <p>The name of the Amazon Kinesis data stream used for the database activity stream.</p>
    ///   - [`status(Option<ActivityStreamStatus>)`](crate::output::StopActivityStreamOutput::status): <p>The status of the database activity stream.</p>
                            /// - On failure, responds with [`SdkError<StopActivityStreamError>`](crate::error::StopActivityStreamError)
    pub fn stop_activity_stream(&self) -> crate::client::fluent_builders::StopActivityStream {
                                crate::client::fluent_builders::StopActivityStream::new(self.handle.clone())
                            }
}

