// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeContinuousExports`](crate::client::fluent_builders::DescribeContinuousExports) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeContinuousExports::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`export_ids(Vec<String>)`](crate::client::fluent_builders::DescribeContinuousExports::export_ids) / [`set_export_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeContinuousExports::set_export_ids): <p>The unique IDs assigned to the exports.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeContinuousExports::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeContinuousExports::set_max_results): <p>A number between 1 and 100 specifying the maximum number of continuous export descriptions returned.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeContinuousExports::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeContinuousExports::set_next_token): <p>The token from the previous call to <code>DescribeExportTasks</code>.</p>
                            /// - On success, responds with [`DescribeContinuousExportsOutput`](crate::output::DescribeContinuousExportsOutput) with field(s):
    ///   - [`descriptions(Option<Vec<ContinuousExportDescription>>)`](crate::output::DescribeContinuousExportsOutput::descriptions): <p>A list of continuous export descriptions.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeContinuousExportsOutput::next_token): <p>The token from the previous call to <code>DescribeExportTasks</code>.</p>
                            /// - On failure, responds with [`SdkError<DescribeContinuousExportsError>`](crate::error::DescribeContinuousExportsError)
    pub fn describe_continuous_exports(&self) -> crate::client::fluent_builders::DescribeContinuousExports {
                                crate::client::fluent_builders::DescribeContinuousExports::new(self.handle.clone())
                            }
}

