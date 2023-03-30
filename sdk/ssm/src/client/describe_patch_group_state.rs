// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribePatchGroupState`](crate::client::fluent_builders::DescribePatchGroupState) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`patch_group(impl Into<String>)`](crate::client::fluent_builders::DescribePatchGroupState::patch_group) / [`set_patch_group(Option<String>)`](crate::client::fluent_builders::DescribePatchGroupState::set_patch_group): <p>The name of the patch group whose patch snapshot should be retrieved.</p>
                            /// - On success, responds with [`DescribePatchGroupStateOutput`](crate::output::DescribePatchGroupStateOutput) with field(s):
    ///   - [`instances(i32)`](crate::output::DescribePatchGroupStateOutput::instances): <p>The number of managed nodes in the patch group.</p>
    ///   - [`instances_with_installed_patches(i32)`](crate::output::DescribePatchGroupStateOutput::instances_with_installed_patches): <p>The number of managed nodes with installed patches.</p>
    ///   - [`instances_with_installed_other_patches(i32)`](crate::output::DescribePatchGroupStateOutput::instances_with_installed_other_patches): <p>The number of managed nodes with patches installed that aren't defined in the patch baseline.</p>
    ///   - [`instances_with_installed_pending_reboot_patches(Option<i32>)`](crate::output::DescribePatchGroupStateOutput::instances_with_installed_pending_reboot_patches): <p>The number of managed nodes with patches installed by Patch Manager that haven't been rebooted after the patch installation. The status of these managed nodes is <code>NON_COMPLIANT</code>.</p>
    ///   - [`instances_with_installed_rejected_patches(Option<i32>)`](crate::output::DescribePatchGroupStateOutput::instances_with_installed_rejected_patches): <p>The number of managed nodes with patches installed that are specified in a <code>RejectedPatches</code> list. Patches with a status of <code>INSTALLED_REJECTED</code> were typically installed before they were added to a <code>RejectedPatches</code> list.</p> <note>   <p>If <code>ALLOW_AS_DEPENDENCY</code> is the specified option for <code>RejectedPatchesAction</code>, the value of <code>InstancesWithInstalledRejectedPatches</code> will always be <code>0</code> (zero).</p>  </note>
    ///   - [`instances_with_missing_patches(i32)`](crate::output::DescribePatchGroupStateOutput::instances_with_missing_patches): <p>The number of managed nodes with missing patches from the patch baseline.</p>
    ///   - [`instances_with_failed_patches(i32)`](crate::output::DescribePatchGroupStateOutput::instances_with_failed_patches): <p>The number of managed nodes with patches from the patch baseline that failed to install.</p>
    ///   - [`instances_with_not_applicable_patches(i32)`](crate::output::DescribePatchGroupStateOutput::instances_with_not_applicable_patches): <p>The number of managed nodes with patches that aren't applicable.</p>
    ///   - [`instances_with_unreported_not_applicable_patches(Option<i32>)`](crate::output::DescribePatchGroupStateOutput::instances_with_unreported_not_applicable_patches): <p>The number of managed nodes with <code>NotApplicable</code> patches beyond the supported limit, which aren't reported by name to Inventory. Inventory is a capability of Amazon Web Services Systems Manager.</p>
    ///   - [`instances_with_critical_non_compliant_patches(Option<i32>)`](crate::output::DescribePatchGroupStateOutput::instances_with_critical_non_compliant_patches): <p>The number of managed nodes where patches that are specified as <code>Critical</code> for compliance reporting in the patch baseline aren't installed. These patches might be missing, have failed installation, were rejected, or were installed but awaiting a required managed node reboot. The status of these managed nodes is <code>NON_COMPLIANT</code>.</p>
    ///   - [`instances_with_security_non_compliant_patches(Option<i32>)`](crate::output::DescribePatchGroupStateOutput::instances_with_security_non_compliant_patches): <p>The number of managed nodes where patches that are specified as <code>Security</code> in a patch advisory aren't installed. These patches might be missing, have failed installation, were rejected, or were installed but awaiting a required managed node reboot. The status of these managed nodes is <code>NON_COMPLIANT</code>.</p>
    ///   - [`instances_with_other_non_compliant_patches(Option<i32>)`](crate::output::DescribePatchGroupStateOutput::instances_with_other_non_compliant_patches): <p>The number of managed nodes with patches installed that are specified as other than <code>Critical</code> or <code>Security</code> but aren't compliant with the patch baseline. The status of these managed nodes is <code>NON_COMPLIANT</code>.</p>
                            /// - On failure, responds with [`SdkError<DescribePatchGroupStateError>`](crate::error::DescribePatchGroupStateError)
    pub fn describe_patch_group_state(&self) -> crate::client::fluent_builders::DescribePatchGroupState {
                                crate::client::fluent_builders::DescribePatchGroupState::new(self.handle.clone())
                            }
}

