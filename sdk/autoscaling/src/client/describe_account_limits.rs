// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAccountLimits`](crate::client::fluent_builders::DescribeAccountLimits) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DescribeAccountLimits::send) it.
                            /// - On success, responds with [`DescribeAccountLimitsOutput`](crate::output::DescribeAccountLimitsOutput) with field(s):
    ///   - [`max_number_of_auto_scaling_groups(Option<i32>)`](crate::output::DescribeAccountLimitsOutput::max_number_of_auto_scaling_groups): <p>The maximum number of groups allowed for your account. The default is 200 groups per Region.</p>
    ///   - [`max_number_of_launch_configurations(Option<i32>)`](crate::output::DescribeAccountLimitsOutput::max_number_of_launch_configurations): <p>The maximum number of launch configurations allowed for your account. The default is 200 launch configurations per Region.</p>
    ///   - [`number_of_auto_scaling_groups(Option<i32>)`](crate::output::DescribeAccountLimitsOutput::number_of_auto_scaling_groups): <p>The current number of groups for your account.</p>
    ///   - [`number_of_launch_configurations(Option<i32>)`](crate::output::DescribeAccountLimitsOutput::number_of_launch_configurations): <p>The current number of launch configurations for your account.</p>
                            /// - On failure, responds with [`SdkError<DescribeAccountLimitsError>`](crate::error::DescribeAccountLimitsError)
    pub fn describe_account_limits(&self) -> crate::client::fluent_builders::DescribeAccountLimits {
                                crate::client::fluent_builders::DescribeAccountLimits::new(self.handle.clone())
                            }
}

