// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeSpotDatafeedSubscription`](crate::client::fluent_builders::DescribeSpotDatafeedSubscription) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeSpotDatafeedSubscription::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeSpotDatafeedSubscription::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DescribeSpotDatafeedSubscriptionOutput`](crate::output::DescribeSpotDatafeedSubscriptionOutput) with field(s):
    ///   - [`spot_datafeed_subscription(Option<SpotDatafeedSubscription>)`](crate::output::DescribeSpotDatafeedSubscriptionOutput::spot_datafeed_subscription): <p>The Spot Instance data feed subscription.</p>
                            /// - On failure, responds with [`SdkError<DescribeSpotDatafeedSubscriptionError>`](crate::error::DescribeSpotDatafeedSubscriptionError)
    pub fn describe_spot_datafeed_subscription(&self) -> crate::client::fluent_builders::DescribeSpotDatafeedSubscription {
                                crate::client::fluent_builders::DescribeSpotDatafeedSubscription::new(self.handle.clone())
                            }
}

