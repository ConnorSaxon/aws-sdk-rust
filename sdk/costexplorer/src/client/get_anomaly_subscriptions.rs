// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetAnomalySubscriptions`](crate::client::fluent_builders::GetAnomalySubscriptions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`subscription_arn_list(Vec<String>)`](crate::client::fluent_builders::GetAnomalySubscriptions::subscription_arn_list) / [`set_subscription_arn_list(Option<Vec<String>>)`](crate::client::fluent_builders::GetAnomalySubscriptions::set_subscription_arn_list): <p>A list of cost anomaly subscription ARNs. </p>
    ///   - [`monitor_arn(impl Into<String>)`](crate::client::fluent_builders::GetAnomalySubscriptions::monitor_arn) / [`set_monitor_arn(Option<String>)`](crate::client::fluent_builders::GetAnomalySubscriptions::set_monitor_arn): <p>Cost anomaly monitor ARNs. </p>
    ///   - [`next_page_token(impl Into<String>)`](crate::client::fluent_builders::GetAnomalySubscriptions::next_page_token) / [`set_next_page_token(Option<String>)`](crate::client::fluent_builders::GetAnomalySubscriptions::set_next_page_token): <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetAnomalySubscriptions::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetAnomalySubscriptions::set_max_results): <p>The number of entries a paginated response contains. </p>
                            /// - On success, responds with [`GetAnomalySubscriptionsOutput`](crate::output::GetAnomalySubscriptionsOutput) with field(s):
    ///   - [`anomaly_subscriptions(Option<Vec<AnomalySubscription>>)`](crate::output::GetAnomalySubscriptionsOutput::anomaly_subscriptions): <p>A list of cost anomaly subscriptions that includes the detailed metadata for each one. </p>
    ///   - [`next_page_token(Option<String>)`](crate::output::GetAnomalySubscriptionsOutput::next_page_token): <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size. </p>
                            /// - On failure, responds with [`SdkError<GetAnomalySubscriptionsError>`](crate::error::GetAnomalySubscriptionsError)
    pub fn get_anomaly_subscriptions(&self) -> crate::client::fluent_builders::GetAnomalySubscriptions {
                                crate::client::fluent_builders::GetAnomalySubscriptions::new(self.handle.clone())
                            }
}

