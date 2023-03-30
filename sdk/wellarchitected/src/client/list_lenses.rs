// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListLenses`](crate::client::fluent_builders::ListLenses) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListLenses::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListLenses::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListLenses::set_next_token): <p>The token to use to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListLenses::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListLenses::set_max_results): <p>The maximum number of results to return for this request.</p>
    ///   - [`lens_type(LensType)`](crate::client::fluent_builders::ListLenses::lens_type) / [`set_lens_type(Option<LensType>)`](crate::client::fluent_builders::ListLenses::set_lens_type): <p>The type of lenses to be returned.</p>
    ///   - [`lens_status(LensStatusType)`](crate::client::fluent_builders::ListLenses::lens_status) / [`set_lens_status(Option<LensStatusType>)`](crate::client::fluent_builders::ListLenses::set_lens_status): <p>The status of lenses to be returned.</p>
    ///   - [`lens_name(impl Into<String>)`](crate::client::fluent_builders::ListLenses::lens_name) / [`set_lens_name(Option<String>)`](crate::client::fluent_builders::ListLenses::set_lens_name): <p>The full name of the lens.</p>
                            /// - On success, responds with [`ListLensesOutput`](crate::output::ListLensesOutput) with field(s):
    ///   - [`lens_summaries(Option<Vec<LensSummary>>)`](crate::output::ListLensesOutput::lens_summaries): <p>List of lens summaries of available lenses.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListLensesOutput::next_token): <p>The token to use to retrieve the next set of results.</p>
                            /// - On failure, responds with [`SdkError<ListLensesError>`](crate::error::ListLensesError)
    pub fn list_lenses(&self) -> crate::client::fluent_builders::ListLenses {
                                crate::client::fluent_builders::ListLenses::new(self.handle.clone())
                            }
}

