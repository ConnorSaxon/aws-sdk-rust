// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListViews`](crate::client::fluent_builders::ListViews) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListViews::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListViews::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListViews::set_next_token): <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListViews::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListViews::set_max_results): <p>The maximum number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value appropriate to the operation. If additional items exist beyond those included in the current response, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results.</p> <note>   <p>An API operation can return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>  </note>
                            /// - On success, responds with [`ListViewsOutput`](crate::output::ListViewsOutput) with field(s):
    ///   - [`views(Option<Vec<String>>)`](crate::output::ListViewsOutput::views): <p>The list of views available in the Amazon Web Services Region in which you called this operation.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListViewsOutput::next_token): <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
                            /// - On failure, responds with [`SdkError<ListViewsError>`](crate::error::ListViewsError)
    pub fn list_views(&self) -> crate::client::fluent_builders::ListViews {
                                crate::client::fluent_builders::ListViews::new(self.handle.clone())
                            }
}

