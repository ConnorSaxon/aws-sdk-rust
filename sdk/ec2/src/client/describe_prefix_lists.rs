// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribePrefixLists`](crate::client::fluent_builders::DescribePrefixLists) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribePrefixLists::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribePrefixLists::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribePrefixLists::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribePrefixLists::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribePrefixLists::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>prefix-list-id</code>: The ID of a prefix list.</p> </li>   <li> <p> <code>prefix-list-name</code>: The name of a prefix list.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribePrefixLists::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribePrefixLists::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribePrefixLists::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribePrefixLists::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`prefix_list_ids(Vec<String>)`](crate::client::fluent_builders::DescribePrefixLists::prefix_list_ids) / [`set_prefix_list_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribePrefixLists::set_prefix_list_ids): <p>One or more prefix list IDs.</p>
                            /// - On success, responds with [`DescribePrefixListsOutput`](crate::output::DescribePrefixListsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::DescribePrefixListsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    ///   - [`prefix_lists(Option<Vec<PrefixList>>)`](crate::output::DescribePrefixListsOutput::prefix_lists): <p>All available prefix lists.</p>
                            /// - On failure, responds with [`SdkError<DescribePrefixListsError>`](crate::error::DescribePrefixListsError)
    pub fn describe_prefix_lists(&self) -> crate::client::fluent_builders::DescribePrefixLists {
                                crate::client::fluent_builders::DescribePrefixLists::new(self.handle.clone())
                            }
}

