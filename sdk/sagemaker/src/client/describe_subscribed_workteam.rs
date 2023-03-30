// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeSubscribedWorkteam`](crate::client::fluent_builders::DescribeSubscribedWorkteam) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workteam_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeSubscribedWorkteam::workteam_arn) / [`set_workteam_arn(Option<String>)`](crate::client::fluent_builders::DescribeSubscribedWorkteam::set_workteam_arn): <p>The Amazon Resource Name (ARN) of the subscribed work team to describe.</p>
                            /// - On success, responds with [`DescribeSubscribedWorkteamOutput`](crate::output::DescribeSubscribedWorkteamOutput) with field(s):
    ///   - [`subscribed_workteam(Option<SubscribedWorkteam>)`](crate::output::DescribeSubscribedWorkteamOutput::subscribed_workteam): <p>A <code>Workteam</code> instance that contains information about the work team.</p>
                            /// - On failure, responds with [`SdkError<DescribeSubscribedWorkteamError>`](crate::error::DescribeSubscribedWorkteamError)
    pub fn describe_subscribed_workteam(&self) -> crate::client::fluent_builders::DescribeSubscribedWorkteam {
                                crate::client::fluent_builders::DescribeSubscribedWorkteam::new(self.handle.clone())
                            }
}

