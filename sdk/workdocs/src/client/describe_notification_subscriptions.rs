// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeNotificationSubscriptions`](crate::client::fluent_builders::DescribeNotificationSubscriptions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::client::fluent_builders::DescribeNotificationSubscriptions::organization_id) / [`set_organization_id(Option<String>)`](crate::client::fluent_builders::DescribeNotificationSubscriptions::set_organization_id): <p>The ID of the organization.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeNotificationSubscriptions::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeNotificationSubscriptions::set_marker): <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribeNotificationSubscriptions::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::DescribeNotificationSubscriptions::set_limit): <p>The maximum number of items to return with this call.</p>
                            /// - On success, responds with [`DescribeNotificationSubscriptionsOutput`](crate::output::DescribeNotificationSubscriptionsOutput) with field(s):
    ///   - [`subscriptions(Option<Vec<Subscription>>)`](crate::output::DescribeNotificationSubscriptionsOutput::subscriptions): <p>The subscriptions.</p>
    ///   - [`marker(Option<String>)`](crate::output::DescribeNotificationSubscriptionsOutput::marker): <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
                            /// - On failure, responds with [`SdkError<DescribeNotificationSubscriptionsError>`](crate::error::DescribeNotificationSubscriptionsError)
    pub fn describe_notification_subscriptions(&self) -> crate::client::fluent_builders::DescribeNotificationSubscriptions {
                                crate::client::fluent_builders::DescribeNotificationSubscriptions::new(self.handle.clone())
                            }
}

