// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetEnabledStandards`](crate::client::fluent_builders::GetEnabledStandards) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetEnabledStandards::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`standards_subscription_arns(Vec<String>)`](crate::client::fluent_builders::GetEnabledStandards::standards_subscription_arns) / [`set_standards_subscription_arns(Option<Vec<String>>)`](crate::client::fluent_builders::GetEnabledStandards::set_standards_subscription_arns): <p>The list of the standards subscription ARNs for the standards to retrieve.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetEnabledStandards::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetEnabledStandards::set_next_token): <p>The token that is required for pagination. On your first call to the <code>GetEnabledStandards</code> operation, set the value of this parameter to <code>NULL</code>.</p>  <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetEnabledStandards::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::GetEnabledStandards::set_max_results): <p>The maximum number of results to return in the response.</p>
                            /// - On success, responds with [`GetEnabledStandardsOutput`](crate::output::GetEnabledStandardsOutput) with field(s):
    ///   - [`standards_subscriptions(Option<Vec<StandardsSubscription>>)`](crate::output::GetEnabledStandardsOutput::standards_subscriptions): <p>The list of <code>StandardsSubscriptions</code> objects that include information about the enabled standards.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetEnabledStandardsOutput::next_token): <p>The pagination token to use to request the next page of results.</p>
                            /// - On failure, responds with [`SdkError<GetEnabledStandardsError>`](crate::error::GetEnabledStandardsError)
    pub fn get_enabled_standards(&self) -> crate::client::fluent_builders::GetEnabledStandards {
                                crate::client::fluent_builders::GetEnabledStandards::new(self.handle.clone())
                            }
}

