// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateRecoveryGroup`](crate::client::fluent_builders::UpdateRecoveryGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cells(Vec<String>)`](crate::client::fluent_builders::UpdateRecoveryGroup::cells) / [`set_cells(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateRecoveryGroup::set_cells): <p>A list of cell Amazon Resource Names (ARNs). This list completely replaces the previous list.</p>
    ///   - [`recovery_group_name(impl Into<String>)`](crate::client::fluent_builders::UpdateRecoveryGroup::recovery_group_name) / [`set_recovery_group_name(Option<String>)`](crate::client::fluent_builders::UpdateRecoveryGroup::set_recovery_group_name): <p>The name of a recovery group.</p>
                            /// - On success, responds with [`UpdateRecoveryGroupOutput`](crate::output::UpdateRecoveryGroupOutput) with field(s):
    ///   - [`cells(Option<Vec<String>>)`](crate::output::UpdateRecoveryGroupOutput::cells): <p>A list of a cell's Amazon Resource Names (ARNs).</p>
    ///   - [`recovery_group_arn(Option<String>)`](crate::output::UpdateRecoveryGroupOutput::recovery_group_arn): <p>The Amazon Resource Name (ARN) for the recovery group.</p>
    ///   - [`recovery_group_name(Option<String>)`](crate::output::UpdateRecoveryGroupOutput::recovery_group_name): <p>The name of the recovery group.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::UpdateRecoveryGroupOutput::tags): <p>The tags associated with the recovery group.</p>
                            /// - On failure, responds with [`SdkError<UpdateRecoveryGroupError>`](crate::error::UpdateRecoveryGroupError)
    pub fn update_recovery_group(&self) -> crate::client::fluent_builders::UpdateRecoveryGroup {
                                crate::client::fluent_builders::UpdateRecoveryGroup::new(self.handle.clone())
                            }
}

