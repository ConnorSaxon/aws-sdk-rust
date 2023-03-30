// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DetectStackSetDrift`](crate::client::fluent_builders::DetectStackSetDrift) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`stack_set_name(impl Into<String>)`](crate::client::fluent_builders::DetectStackSetDrift::stack_set_name) / [`set_stack_set_name(Option<String>)`](crate::client::fluent_builders::DetectStackSetDrift::set_stack_set_name): <p>The name of the stack set on which to perform the drift detection operation.</p>
    ///   - [`operation_preferences(StackSetOperationPreferences)`](crate::client::fluent_builders::DetectStackSetDrift::operation_preferences) / [`set_operation_preferences(Option<StackSetOperationPreferences>)`](crate::client::fluent_builders::DetectStackSetDrift::set_operation_preferences): <p>The user-specified preferences for how CloudFormation performs a stack set operation.</p>  <p>For more information about maximum concurrent accounts and failure tolerance, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-concepts.html#stackset-ops-options">Stack set operation options</a>.</p>
    ///   - [`operation_id(impl Into<String>)`](crate::client::fluent_builders::DetectStackSetDrift::operation_id) / [`set_operation_id(Option<String>)`](crate::client::fluent_builders::DetectStackSetDrift::set_operation_id): <p> <i>The ID of the stack set operation.</i> </p>
    ///   - [`call_as(CallAs)`](crate::client::fluent_builders::DetectStackSetDrift::call_as) / [`set_call_as(Option<CallAs>)`](crate::client::fluent_builders::DetectStackSetDrift::set_call_as): <p>[Service-managed permissions] Specifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account.</p>  <p>By default, <code>SELF</code> is specified. Use <code>SELF</code> for stack sets with self-managed permissions.</p>  <ul>   <li> <p>If you are signed in to the management account, specify <code>SELF</code>.</p> </li>   <li> <p>If you are signed in to a delegated administrator account, specify <code>DELEGATED_ADMIN</code>.</p> <p>Your Amazon Web Services account must be registered as a delegated administrator in the management account. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-orgs-delegated-admin.html">Register a delegated administrator</a> in the <i>CloudFormation User Guide</i>.</p> </li>  </ul>
                            /// - On success, responds with [`DetectStackSetDriftOutput`](crate::output::DetectStackSetDriftOutput) with field(s):
    ///   - [`operation_id(Option<String>)`](crate::output::DetectStackSetDriftOutput::operation_id): <p>The ID of the drift detection stack set operation.</p>  <p>You can use this operation ID with <code> <code>DescribeStackSetOperation</code> </code> to monitor the progress of the drift detection operation.</p>
                            /// - On failure, responds with [`SdkError<DetectStackSetDriftError>`](crate::error::DetectStackSetDriftError)
    pub fn detect_stack_set_drift(&self) -> crate::client::fluent_builders::DetectStackSetDrift {
                                crate::client::fluent_builders::DetectStackSetDrift::new(self.handle.clone())
                            }
}

