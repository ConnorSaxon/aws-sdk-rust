// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteChapCredentials`](crate::client::fluent_builders::DeleteChapCredentials) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`target_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteChapCredentials::target_arn) / [`set_target_arn(Option<String>)`](crate::client::fluent_builders::DeleteChapCredentials::set_target_arn): <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <code>DescribeStorediSCSIVolumes</code> operation to return to retrieve the TargetARN for specified VolumeARN.</p>
    ///   - [`initiator_name(impl Into<String>)`](crate::client::fluent_builders::DeleteChapCredentials::initiator_name) / [`set_initiator_name(Option<String>)`](crate::client::fluent_builders::DeleteChapCredentials::set_initiator_name): <p>The iSCSI initiator that connects to the target.</p>
                            /// - On success, responds with [`DeleteChapCredentialsOutput`](crate::output::DeleteChapCredentialsOutput) with field(s):
    ///   - [`target_arn(Option<String>)`](crate::output::DeleteChapCredentialsOutput::target_arn): <p>The Amazon Resource Name (ARN) of the target.</p>
    ///   - [`initiator_name(Option<String>)`](crate::output::DeleteChapCredentialsOutput::initiator_name): <p>The iSCSI initiator that connects to the target.</p>
                            /// - On failure, responds with [`SdkError<DeleteChapCredentialsError>`](crate::error::DeleteChapCredentialsError)
    pub fn delete_chap_credentials(&self) -> crate::client::fluent_builders::DeleteChapCredentials {
                                crate::client::fluent_builders::DeleteChapCredentials::new(self.handle.clone())
                            }
}

