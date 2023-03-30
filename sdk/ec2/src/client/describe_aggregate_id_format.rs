// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAggregateIdFormat`](crate::client::fluent_builders::DescribeAggregateIdFormat) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeAggregateIdFormat::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeAggregateIdFormat::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`DescribeAggregateIdFormatOutput`](crate::output::DescribeAggregateIdFormatOutput) with field(s):
    ///   - [`use_long_ids_aggregated(Option<bool>)`](crate::output::DescribeAggregateIdFormatOutput::use_long_ids_aggregated): <p>Indicates whether all resource types in the Region are configured to use longer IDs. This value is only <code>true</code> if all users are configured to use longer IDs for all resources types in the Region.</p>
    ///   - [`statuses(Option<Vec<IdFormat>>)`](crate::output::DescribeAggregateIdFormatOutput::statuses): <p>Information about each resource's ID format.</p>
                            /// - On failure, responds with [`SdkError<DescribeAggregateIdFormatError>`](crate::error::DescribeAggregateIdFormatError)
    pub fn describe_aggregate_id_format(&self) -> crate::client::fluent_builders::DescribeAggregateIdFormat {
                                crate::client::fluent_builders::DescribeAggregateIdFormat::new(self.handle.clone())
                            }
}

