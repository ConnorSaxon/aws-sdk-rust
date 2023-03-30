// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeHapg`](crate::client::fluent_builders::DescribeHapg) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hapg_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeHapg::hapg_arn) / [`set_hapg_arn(Option<String>)`](crate::client::fluent_builders::DescribeHapg::set_hapg_arn): <p>The ARN of the high-availability partition group to describe.</p>
                            /// - On success, responds with [`DescribeHapgOutput`](crate::output::DescribeHapgOutput) with field(s):
    ///   - [`hapg_arn(Option<String>)`](crate::output::DescribeHapgOutput::hapg_arn): <p>The ARN of the high-availability partition group.</p>
    ///   - [`hapg_serial(Option<String>)`](crate::output::DescribeHapgOutput::hapg_serial): <p>The serial number of the high-availability partition group.</p>
    ///   - [`hsms_last_action_failed(Option<Vec<String>>)`](crate::output::DescribeHapgOutput::hsms_last_action_failed): <p></p>
    ///   - [`hsms_pending_deletion(Option<Vec<String>>)`](crate::output::DescribeHapgOutput::hsms_pending_deletion): <p></p>
    ///   - [`hsms_pending_registration(Option<Vec<String>>)`](crate::output::DescribeHapgOutput::hsms_pending_registration): <p></p>
    ///   - [`label(Option<String>)`](crate::output::DescribeHapgOutput::label): <p>The label for the high-availability partition group.</p>
    ///   - [`last_modified_timestamp(Option<String>)`](crate::output::DescribeHapgOutput::last_modified_timestamp): <p>The date and time the high-availability partition group was last modified.</p>
    ///   - [`partition_serial_list(Option<Vec<String>>)`](crate::output::DescribeHapgOutput::partition_serial_list): <p>The list of partition serial numbers that belong to the high-availability partition group.</p>
    ///   - [`state(Option<CloudHsmObjectState>)`](crate::output::DescribeHapgOutput::state): <p>The state of the high-availability partition group.</p>
                            /// - On failure, responds with [`SdkError<DescribeHapgError>`](crate::error::DescribeHapgError)
    pub fn describe_hapg(&self) -> crate::client::fluent_builders::DescribeHapg {
                                crate::client::fluent_builders::DescribeHapg::new(self.handle.clone())
                            }
}

