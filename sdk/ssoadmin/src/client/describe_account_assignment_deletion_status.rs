// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAccountAssignmentDeletionStatus`](crate::client::fluent_builders::DescribeAccountAssignmentDeletionStatus) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeAccountAssignmentDeletionStatus::instance_arn) / [`set_instance_arn(Option<String>)`](crate::client::fluent_builders::DescribeAccountAssignmentDeletionStatus::set_instance_arn): <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    ///   - [`account_assignment_deletion_request_id(impl Into<String>)`](crate::client::fluent_builders::DescribeAccountAssignmentDeletionStatus::account_assignment_deletion_request_id) / [`set_account_assignment_deletion_request_id(Option<String>)`](crate::client::fluent_builders::DescribeAccountAssignmentDeletionStatus::set_account_assignment_deletion_request_id): <p>The identifier that is used to track the request operation progress.</p>
                            /// - On success, responds with [`DescribeAccountAssignmentDeletionStatusOutput`](crate::output::DescribeAccountAssignmentDeletionStatusOutput) with field(s):
    ///   - [`account_assignment_deletion_status(Option<AccountAssignmentOperationStatus>)`](crate::output::DescribeAccountAssignmentDeletionStatusOutput::account_assignment_deletion_status): <p>The status object for the account assignment deletion operation.</p>
                            /// - On failure, responds with [`SdkError<DescribeAccountAssignmentDeletionStatusError>`](crate::error::DescribeAccountAssignmentDeletionStatusError)
    pub fn describe_account_assignment_deletion_status(&self) -> crate::client::fluent_builders::DescribeAccountAssignmentDeletionStatus {
                                crate::client::fluent_builders::DescribeAccountAssignmentDeletionStatus::new(self.handle.clone())
                            }
}

