// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetMLTransforms`](crate::client::fluent_builders::GetMLTransforms) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetMLTransforms::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetMLTransforms::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetMLTransforms::set_next_token): <p>A paginated token to offset the results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetMLTransforms::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetMLTransforms::set_max_results): <p>The maximum number of results to return.</p>
    ///   - [`filter(TransformFilterCriteria)`](crate::client::fluent_builders::GetMLTransforms::filter) / [`set_filter(Option<TransformFilterCriteria>)`](crate::client::fluent_builders::GetMLTransforms::set_filter): <p>The filter transformation criteria.</p>
    ///   - [`sort(TransformSortCriteria)`](crate::client::fluent_builders::GetMLTransforms::sort) / [`set_sort(Option<TransformSortCriteria>)`](crate::client::fluent_builders::GetMLTransforms::set_sort): <p>The sorting criteria.</p>
                            /// - On success, responds with [`GetMlTransformsOutput`](crate::output::GetMlTransformsOutput) with field(s):
    ///   - [`transforms(Option<Vec<MlTransform>>)`](crate::output::GetMlTransformsOutput::transforms): <p>A list of machine learning transforms.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetMlTransformsOutput::next_token): <p>A pagination token, if more results are available.</p>
                            /// - On failure, responds with [`SdkError<GetMLTransformsError>`](crate::error::GetMLTransformsError)
    pub fn get_ml_transforms(&self) -> crate::client::fluent_builders::GetMLTransforms {
                                crate::client::fluent_builders::GetMLTransforms::new(self.handle.clone())
                            }
}

