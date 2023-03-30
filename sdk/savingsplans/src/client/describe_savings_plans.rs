// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeSavingsPlans`](crate::client::fluent_builders::DescribeSavingsPlans) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`savings_plan_arns(Vec<String>)`](crate::client::fluent_builders::DescribeSavingsPlans::savings_plan_arns) / [`set_savings_plan_arns(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeSavingsPlans::set_savings_plan_arns): <p>The Amazon Resource Names (ARN) of the Savings Plans.</p>
    ///   - [`savings_plan_ids(Vec<String>)`](crate::client::fluent_builders::DescribeSavingsPlans::savings_plan_ids) / [`set_savings_plan_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeSavingsPlans::set_savings_plan_ids): <p>The IDs of the Savings Plans.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeSavingsPlans::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeSavingsPlans::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeSavingsPlans::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeSavingsPlans::set_max_results): <p>The maximum number of results to return with a single call. To retrieve additional results, make another call with the returned token value.</p>
    ///   - [`states(Vec<SavingsPlanState>)`](crate::client::fluent_builders::DescribeSavingsPlans::states) / [`set_states(Option<Vec<SavingsPlanState>>)`](crate::client::fluent_builders::DescribeSavingsPlans::set_states): <p>The states.</p>
    ///   - [`filters(Vec<SavingsPlanFilter>)`](crate::client::fluent_builders::DescribeSavingsPlans::filters) / [`set_filters(Option<Vec<SavingsPlanFilter>>)`](crate::client::fluent_builders::DescribeSavingsPlans::set_filters): <p>The filters.</p>
                            /// - On success, responds with [`DescribeSavingsPlansOutput`](crate::output::DescribeSavingsPlansOutput) with field(s):
    ///   - [`savings_plans(Option<Vec<SavingsPlan>>)`](crate::output::DescribeSavingsPlansOutput::savings_plans): <p>Information about the Savings Plans.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeSavingsPlansOutput::next_token): <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeSavingsPlansError>`](crate::error::DescribeSavingsPlansError)
    pub fn describe_savings_plans(&self) -> crate::client::fluent_builders::DescribeSavingsPlans {
                                crate::client::fluent_builders::DescribeSavingsPlans::new(self.handle.clone())
                            }
}

