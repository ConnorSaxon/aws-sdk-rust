// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListWorkGroups`](crate::client::fluent_builders::ListWorkGroups) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListWorkGroups::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListWorkGroups::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListWorkGroups::set_next_token): <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListWorkGroups::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListWorkGroups::set_max_results): <p>The maximum number of workgroups to return in this request.</p>
                            /// - On success, responds with [`ListWorkGroupsOutput`](crate::output::ListWorkGroupsOutput) with field(s):
    ///   - [`work_groups(Option<Vec<WorkGroupSummary>>)`](crate::output::ListWorkGroupsOutput::work_groups): <p>A list of <code>WorkGroupSummary</code> objects that include the names, descriptions, creation times, and states for each workgroup.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListWorkGroupsOutput::next_token): <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
                            /// - On failure, responds with [`SdkError<ListWorkGroupsError>`](crate::error::ListWorkGroupsError)
    pub fn list_work_groups(&self) -> crate::client::fluent_builders::ListWorkGroups {
                                crate::client::fluent_builders::ListWorkGroups::new(self.handle.clone())
                            }
}

