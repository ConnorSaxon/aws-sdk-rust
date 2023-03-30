// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeStandards`](crate::client::fluent_builders::DescribeStandards) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeStandards::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeStandards::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeStandards::set_next_token): <p>The token that is required for pagination. On your first call to the <code>DescribeStandards</code> operation, set the value of this parameter to <code>NULL</code>.</p>  <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeStandards::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::DescribeStandards::set_max_results): <p>The maximum number of standards to return.</p>
                            /// - On success, responds with [`DescribeStandardsOutput`](crate::output::DescribeStandardsOutput) with field(s):
    ///   - [`standards(Option<Vec<Standard>>)`](crate::output::DescribeStandardsOutput::standards): <p>A list of available standards.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeStandardsOutput::next_token): <p>The pagination token to use to request the next page of results.</p>
                            /// - On failure, responds with [`SdkError<DescribeStandardsError>`](crate::error::DescribeStandardsError)
    pub fn describe_standards(&self) -> crate::client::fluent_builders::DescribeStandards {
                                crate::client::fluent_builders::DescribeStandards::new(self.handle.clone())
                            }
}

