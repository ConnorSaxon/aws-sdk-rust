// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateBillingGroup`](crate::client::fluent_builders::UpdateBillingGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::UpdateBillingGroup::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::UpdateBillingGroup::set_arn): <p>The Amazon Resource Name (ARN) of the billing group being updated. </p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateBillingGroup::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateBillingGroup::set_name): <p>The name of the billing group. The names must be unique to each billing group. </p>
    ///   - [`status(BillingGroupStatus)`](crate::client::fluent_builders::UpdateBillingGroup::status) / [`set_status(Option<BillingGroupStatus>)`](crate::client::fluent_builders::UpdateBillingGroup::set_status): <p>The status of the billing group. Only one of the valid values can be used. </p>
    ///   - [`computation_preference(ComputationPreference)`](crate::client::fluent_builders::UpdateBillingGroup::computation_preference) / [`set_computation_preference(Option<ComputationPreference>)`](crate::client::fluent_builders::UpdateBillingGroup::set_computation_preference): <p> The preferences and settings that will be used to compute the Amazon Web Services charges for a billing group. </p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateBillingGroup::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateBillingGroup::set_description): <p>A description of the billing group. </p>
                            /// - On success, responds with [`UpdateBillingGroupOutput`](crate::output::UpdateBillingGroupOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::UpdateBillingGroupOutput::arn): <p>The Amazon Resource Name (ARN) of the billing group that was updated. </p>
    ///   - [`name(Option<String>)`](crate::output::UpdateBillingGroupOutput::name): <p> The name of the billing group. The names must be unique to each billing group. </p>
    ///   - [`description(Option<String>)`](crate::output::UpdateBillingGroupOutput::description): <p> A description of the billing group. </p>
    ///   - [`primary_account_id(Option<String>)`](crate::output::UpdateBillingGroupOutput::primary_account_id): <p> The account ID that serves as the main account in a billing group. </p>
    ///   - [`pricing_plan_arn(Option<String>)`](crate::output::UpdateBillingGroupOutput::pricing_plan_arn): <p> The Amazon Resource Name (ARN) of the pricing plan to compute Amazon Web Services charges for the billing group. </p>
    ///   - [`size(i64)`](crate::output::UpdateBillingGroupOutput::size): <p> The number of accounts in the particular billing group. </p>
    ///   - [`last_modified_time(i64)`](crate::output::UpdateBillingGroupOutput::last_modified_time): <p> The most recent time when the billing group was modified. </p>
    ///   - [`status(Option<BillingGroupStatus>)`](crate::output::UpdateBillingGroupOutput::status): <p> The status of the billing group. Only one of the valid values can be used. </p>
    ///   - [`status_reason(Option<String>)`](crate::output::UpdateBillingGroupOutput::status_reason): <p> The reason why the billing group is in its current status. </p>
                            /// - On failure, responds with [`SdkError<UpdateBillingGroupError>`](crate::error::UpdateBillingGroupError)
    pub fn update_billing_group(&self) -> crate::client::fluent_builders::UpdateBillingGroup {
                                crate::client::fluent_builders::UpdateBillingGroup::new(self.handle.clone())
                            }
}

