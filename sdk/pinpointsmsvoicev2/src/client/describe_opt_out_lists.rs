// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeOptOutLists`](crate::client::fluent_builders::DescribeOptOutLists) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeOptOutLists::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`opt_out_list_names(Vec<String>)`](crate::client::fluent_builders::DescribeOptOutLists::opt_out_list_names) / [`set_opt_out_list_names(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeOptOutLists::set_opt_out_list_names): <p>The OptOutLists to show the details of. This is an array of strings that can be either the OptOutListName or OptOutListArn.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeOptOutLists::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeOptOutLists::set_next_token): <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeOptOutLists::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeOptOutLists::set_max_results): <p>The maximum number of results to return per each request.</p>
                            /// - On success, responds with [`DescribeOptOutListsOutput`](crate::output::DescribeOptOutListsOutput) with field(s):
    ///   - [`opt_out_lists(Option<Vec<OptOutListInformation>>)`](crate::output::DescribeOptOutListsOutput::opt_out_lists): <p>An array of OptOutListInformation objects that contain the details for the requested OptOutLists.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeOptOutListsOutput::next_token): <p>The token to be used for the next set of paginated results. If this field is empty then there are no more results.</p>
                            /// - On failure, responds with [`SdkError<DescribeOptOutListsError>`](crate::error::DescribeOptOutListsError)
    pub fn describe_opt_out_lists(&self) -> crate::client::fluent_builders::DescribeOptOutLists {
                                crate::client::fluent_builders::DescribeOptOutLists::new(self.handle.clone())
                            }
}

