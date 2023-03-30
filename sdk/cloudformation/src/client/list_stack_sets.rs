// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListStackSets`](crate::client::fluent_builders::ListStackSets) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListStackSets::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListStackSets::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListStackSets::set_next_token): <p>If the previous paginated request didn't return all the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListStackSets</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListStackSets::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListStackSets::set_max_results): <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    ///   - [`status(StackSetStatus)`](crate::client::fluent_builders::ListStackSets::status) / [`set_status(Option<StackSetStatus>)`](crate::client::fluent_builders::ListStackSets::set_status): <p>The status of the stack sets that you want to get summary information about.</p>
    ///   - [`call_as(CallAs)`](crate::client::fluent_builders::ListStackSets::call_as) / [`set_call_as(Option<CallAs>)`](crate::client::fluent_builders::ListStackSets::set_call_as): <p>[Service-managed permissions] Specifies whether you are acting as an account administrator in the management account or as a delegated administrator in a member account.</p>  <p>By default, <code>SELF</code> is specified. Use <code>SELF</code> for stack sets with self-managed permissions.</p>  <ul>   <li> <p>If you are signed in to the management account, specify <code>SELF</code>.</p> </li>   <li> <p>If you are signed in to a delegated administrator account, specify <code>DELEGATED_ADMIN</code>.</p> <p>Your Amazon Web Services account must be registered as a delegated administrator in the management account. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-orgs-delegated-admin.html">Register a delegated administrator</a> in the <i>CloudFormation User Guide</i>.</p> </li>  </ul>
                            /// - On success, responds with [`ListStackSetsOutput`](crate::output::ListStackSetsOutput) with field(s):
    ///   - [`summaries(Option<Vec<StackSetSummary>>)`](crate::output::ListStackSetsOutput::summaries): <p>A list of <code>StackSetSummary</code> structures that contain information about the user's stack sets.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListStackSetsOutput::next_token): <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListStackInstances</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to <code>null</code>.</p>
                            /// - On failure, responds with [`SdkError<ListStackSetsError>`](crate::error::ListStackSetsError)
    pub fn list_stack_sets(&self) -> crate::client::fluent_builders::ListStackSets {
                                crate::client::fluent_builders::ListStackSets::new(self.handle.clone())
                            }
}

