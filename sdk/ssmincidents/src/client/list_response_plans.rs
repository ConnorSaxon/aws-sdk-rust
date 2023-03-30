// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListResponsePlans`](crate::client::fluent_builders::ListResponsePlans) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListResponsePlans::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListResponsePlans::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListResponsePlans::set_max_results): <p>The maximum number of response plans per page.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListResponsePlans::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListResponsePlans::set_next_token): <p>The pagination token to continue to the next page of results.</p>
                            /// - On success, responds with [`ListResponsePlansOutput`](crate::output::ListResponsePlansOutput) with field(s):
    ///   - [`response_plan_summaries(Option<Vec<ResponsePlanSummary>>)`](crate::output::ListResponsePlansOutput::response_plan_summaries): <p>Details of each response plan.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListResponsePlansOutput::next_token): <p>The pagination token to continue to the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListResponsePlansError>`](crate::error::ListResponsePlansError)
    pub fn list_response_plans(&self) -> crate::client::fluent_builders::ListResponsePlans {
                                crate::client::fluent_builders::ListResponsePlans::new(self.handle.clone())
                            }
}

