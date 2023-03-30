// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeActivations`](crate::client::fluent_builders::DescribeActivations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeActivations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filters(Vec<DescribeActivationsFilter>)`](crate::client::fluent_builders::DescribeActivations::filters) / [`set_filters(Option<Vec<DescribeActivationsFilter>>)`](crate::client::fluent_builders::DescribeActivations::set_filters): <p>A filter to view information about your activations.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeActivations::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeActivations::set_max_results): <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeActivations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeActivations::set_next_token): <p>A token to start the list. Use this token to get the next set of results. </p>
                            /// - On success, responds with [`DescribeActivationsOutput`](crate::output::DescribeActivationsOutput) with field(s):
    ///   - [`activation_list(Option<Vec<Activation>>)`](crate::output::DescribeActivationsOutput::activation_list): <p>A list of activations for your Amazon Web Services account.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeActivationsOutput::next_token): <p>The token for the next set of items to return. Use this token to get the next set of results. </p>
                            /// - On failure, responds with [`SdkError<DescribeActivationsError>`](crate::error::DescribeActivationsError)
    pub fn describe_activations(&self) -> crate::client::fluent_builders::DescribeActivations {
                                crate::client::fluent_builders::DescribeActivations::new(self.handle.clone())
                            }
}

