// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRecoveryGroupReadinessSummary`](crate::client::fluent_builders::GetRecoveryGroupReadinessSummary) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetRecoveryGroupReadinessSummary::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetRecoveryGroupReadinessSummary::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::GetRecoveryGroupReadinessSummary::set_max_results): <p>The number of objects that you want to return with this call.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetRecoveryGroupReadinessSummary::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetRecoveryGroupReadinessSummary::set_next_token): <p>The token that identifies which batch of results you want to see.</p>
    ///   - [`recovery_group_name(impl Into<String>)`](crate::client::fluent_builders::GetRecoveryGroupReadinessSummary::recovery_group_name) / [`set_recovery_group_name(Option<String>)`](crate::client::fluent_builders::GetRecoveryGroupReadinessSummary::set_recovery_group_name): <p>The name of a recovery group.</p>
                            /// - On success, responds with [`GetRecoveryGroupReadinessSummaryOutput`](crate::output::GetRecoveryGroupReadinessSummaryOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::GetRecoveryGroupReadinessSummaryOutput::next_token): <p>The token that identifies which batch of results you want to see.</p>
    ///   - [`readiness(Option<Readiness>)`](crate::output::GetRecoveryGroupReadinessSummaryOutput::readiness): <p>The readiness status at a recovery group level.</p>
    ///   - [`readiness_checks(Option<Vec<ReadinessCheckSummary>>)`](crate::output::GetRecoveryGroupReadinessSummaryOutput::readiness_checks): <p>Summaries of the readiness checks for the recovery group.</p>
                            /// - On failure, responds with [`SdkError<GetRecoveryGroupReadinessSummaryError>`](crate::error::GetRecoveryGroupReadinessSummaryError)
    pub fn get_recovery_group_readiness_summary(&self) -> crate::client::fluent_builders::GetRecoveryGroupReadinessSummary {
                                crate::client::fluent_builders::GetRecoveryGroupReadinessSummary::new(self.handle.clone())
                            }
}

