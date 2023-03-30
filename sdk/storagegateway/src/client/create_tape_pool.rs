// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateTapePool`](crate::client::fluent_builders::CreateTapePool) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`pool_name(impl Into<String>)`](crate::client::fluent_builders::CreateTapePool::pool_name) / [`set_pool_name(Option<String>)`](crate::client::fluent_builders::CreateTapePool::set_pool_name): <p>The name of the new custom tape pool.</p>
    ///   - [`storage_class(TapeStorageClass)`](crate::client::fluent_builders::CreateTapePool::storage_class) / [`set_storage_class(Option<TapeStorageClass>)`](crate::client::fluent_builders::CreateTapePool::set_storage_class): <p>The storage class that is associated with the new custom pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p>
    ///   - [`retention_lock_type(RetentionLockType)`](crate::client::fluent_builders::CreateTapePool::retention_lock_type) / [`set_retention_lock_type(Option<RetentionLockType>)`](crate::client::fluent_builders::CreateTapePool::set_retention_lock_type): <p>Tape retention lock can be configured in two modes. When configured in governance mode, Amazon Web Services accounts with specific IAM permissions are authorized to remove the tape retention lock from archived virtual tapes. When configured in compliance mode, the tape retention lock cannot be removed by any user, including the root Amazon Web Services account.</p>
    ///   - [`retention_lock_time_in_days(i32)`](crate::client::fluent_builders::CreateTapePool::retention_lock_time_in_days) / [`set_retention_lock_time_in_days(Option<i32>)`](crate::client::fluent_builders::CreateTapePool::set_retention_lock_time_in_days): <p>Tape retention lock time is set in days. Tape retention lock can be enabled for up to 100 years (36,500 days).</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateTapePool::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateTapePool::set_tags): <p>A list of up to 50 tags that can be assigned to tape pool. Each tag is a key-value pair.</p> <note>   <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag's key is 128 characters, and the maximum length for a tag's value is 256.</p>  </note>
                            /// - On success, responds with [`CreateTapePoolOutput`](crate::output::CreateTapePoolOutput) with field(s):
    ///   - [`pool_arn(Option<String>)`](crate::output::CreateTapePoolOutput::pool_arn): <p>The unique Amazon Resource Name (ARN) that represents the custom tape pool. Use the <code>ListTapePools</code> operation to return a list of tape pools for your account and Amazon Web Services Region.</p>
                            /// - On failure, responds with [`SdkError<CreateTapePoolError>`](crate::error::CreateTapePoolError)
    pub fn create_tape_pool(&self) -> crate::client::fluent_builders::CreateTapePool {
                                crate::client::fluent_builders::CreateTapePool::new(self.handle.clone())
                            }
}

