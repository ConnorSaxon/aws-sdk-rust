// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeInstanceEventWindows`](crate::client::fluent_builders::DescribeInstanceEventWindows) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeInstanceEventWindows::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeInstanceEventWindows::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeInstanceEventWindows::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_event_window_ids(Vec<String>)`](crate::client::fluent_builders::DescribeInstanceEventWindows::instance_event_window_ids) / [`set_instance_event_window_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeInstanceEventWindows::set_instance_event_window_ids): <p>The IDs of the event windows.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeInstanceEventWindows::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeInstanceEventWindows::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>dedicated-host-id</code> - The event windows associated with the specified Dedicated Host ID.</p> </li>   <li> <p> <code>event-window-name</code> - The event windows associated with the specified names. </p> </li>   <li> <p> <code>instance-id</code> - The event windows associated with the specified instance ID.</p> </li>   <li> <p> <code>instance-tag</code> - The event windows associated with the specified tag and value.</p> </li>   <li> <p> <code>instance-tag-key</code> - The event windows associated with the specified tag key, regardless of the value.</p> </li>   <li> <p> <code>instance-tag-value</code> - The event windows associated with the specified tag value, regardless of the key.</p> </li>   <li> <p> <code>tag:     <key></key></code> - The key/value combination of a tag assigned to the event window. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>CMX</code>, specify <code>tag:Owner</code> for the filter name and <code>CMX</code> for the filter value. </p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the event window. Use this filter to find all event windows that have a tag with a specific key, regardless of the tag value. </p> </li>   <li> <p> <code>tag-value</code> - The value of a tag assigned to the event window. Use this filter to find all event windows that have a tag with a specific value, regardless of the tag key. </p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeInstanceEventWindows::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeInstanceEventWindows::set_max_results): <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value. This value can be between 20 and 500. You cannot specify this parameter and the event window IDs parameter in the same call.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeInstanceEventWindows::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeInstanceEventWindows::set_next_token): <p>The token to request the next page of results.</p>
                            /// - On success, responds with [`DescribeInstanceEventWindowsOutput`](crate::output::DescribeInstanceEventWindowsOutput) with field(s):
    ///   - [`instance_event_windows(Option<Vec<InstanceEventWindow>>)`](crate::output::DescribeInstanceEventWindowsOutput::instance_event_windows): <p>Information about the event windows.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeInstanceEventWindowsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
                            /// - On failure, responds with [`SdkError<DescribeInstanceEventWindowsError>`](crate::error::DescribeInstanceEventWindowsError)
    pub fn describe_instance_event_windows(&self) -> crate::client::fluent_builders::DescribeInstanceEventWindows {
                                crate::client::fluent_builders::DescribeInstanceEventWindows::new(self.handle.clone())
                            }
}

