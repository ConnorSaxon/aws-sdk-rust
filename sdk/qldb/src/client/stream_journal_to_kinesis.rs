// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StreamJournalToKinesis`](crate::client::fluent_builders::StreamJournalToKinesis) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`ledger_name(impl Into<String>)`](crate::client::fluent_builders::StreamJournalToKinesis::ledger_name) / [`set_ledger_name(Option<String>)`](crate::client::fluent_builders::StreamJournalToKinesis::set_ledger_name): <p>The name of the ledger.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::StreamJournalToKinesis::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::StreamJournalToKinesis::set_role_arn): <p>The Amazon Resource Name (ARN) of the IAM role that grants QLDB permissions for a journal stream to write data records to a Kinesis Data Streams resource.</p>  <p>To pass a role to QLDB when requesting a journal stream, you must have permissions to perform the <code>iam:PassRole</code> action on the IAM role resource. This is required for all journal stream requests.</p>
    ///   - [`tags(HashMap<String, Option<String>>)`](crate::client::fluent_builders::StreamJournalToKinesis::tags) / [`set_tags(Option<HashMap<String, Option<String>>>)`](crate::client::fluent_builders::StreamJournalToKinesis::set_tags): <p>The key-value pairs to add as tags to the stream that you want to create. Tag keys are case sensitive. Tag values are case sensitive and can be null.</p>
    ///   - [`inclusive_start_time(DateTime)`](crate::client::fluent_builders::StreamJournalToKinesis::inclusive_start_time) / [`set_inclusive_start_time(Option<DateTime>)`](crate::client::fluent_builders::StreamJournalToKinesis::set_inclusive_start_time): <p>The inclusive start date and time from which to start streaming journal data. This parameter must be in <code>ISO 8601</code> date and time format and in Universal Coordinated Time (UTC). For example: <code>2019-06-13T21:36:34Z</code>.</p>  <p>The <code>InclusiveStartTime</code> cannot be in the future and must be before <code>ExclusiveEndTime</code>.</p>  <p>If you provide an <code>InclusiveStartTime</code> that is before the ledger's <code>CreationDateTime</code>, QLDB effectively defaults it to the ledger's <code>CreationDateTime</code>.</p>
    ///   - [`exclusive_end_time(DateTime)`](crate::client::fluent_builders::StreamJournalToKinesis::exclusive_end_time) / [`set_exclusive_end_time(Option<DateTime>)`](crate::client::fluent_builders::StreamJournalToKinesis::set_exclusive_end_time): <p>The exclusive date and time that specifies when the stream ends. If you don't define this parameter, the stream runs indefinitely until you cancel it.</p>  <p>The <code>ExclusiveEndTime</code> must be in <code>ISO 8601</code> date and time format and in Universal Coordinated Time (UTC). For example: <code>2019-06-13T21:36:34Z</code>.</p>
    ///   - [`kinesis_configuration(KinesisConfiguration)`](crate::client::fluent_builders::StreamJournalToKinesis::kinesis_configuration) / [`set_kinesis_configuration(Option<KinesisConfiguration>)`](crate::client::fluent_builders::StreamJournalToKinesis::set_kinesis_configuration): <p>The configuration settings of the Kinesis Data Streams destination for your stream request.</p>
    ///   - [`stream_name(impl Into<String>)`](crate::client::fluent_builders::StreamJournalToKinesis::stream_name) / [`set_stream_name(Option<String>)`](crate::client::fluent_builders::StreamJournalToKinesis::set_stream_name): <p>The name that you want to assign to the QLDB journal stream. User-defined names can help identify and indicate the purpose of a stream.</p>  <p>Your stream name must be unique among other <i>active</i> streams for a given ledger. Stream names have the same naming constraints as ledger names, as defined in <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/limits.html#limits.naming">Quotas in Amazon QLDB</a> in the <i>Amazon QLDB Developer Guide</i>.</p>
                            /// - On success, responds with [`StreamJournalToKinesisOutput`](crate::output::StreamJournalToKinesisOutput) with field(s):
    ///   - [`stream_id(Option<String>)`](crate::output::StreamJournalToKinesisOutput::stream_id): <p>The UUID (represented in Base62-encoded text) that QLDB assigns to each QLDB journal stream.</p>
                            /// - On failure, responds with [`SdkError<StreamJournalToKinesisError>`](crate::error::StreamJournalToKinesisError)
    pub fn stream_journal_to_kinesis(&self) -> crate::client::fluent_builders::StreamJournalToKinesis {
                                crate::client::fluent_builders::StreamJournalToKinesis::new(self.handle.clone())
                            }
}

