// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAnalyzers`](crate::client::fluent_builders::ListAnalyzers) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAnalyzers::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAnalyzers::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAnalyzers::set_next_token): <p>A token used for pagination of results returned.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAnalyzers::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAnalyzers::set_max_results): <p>The maximum number of results to return in the response.</p>
    ///   - [`r#type(Type)`](crate::client::fluent_builders::ListAnalyzers::type) / [`set_type(Option<Type>)`](crate::client::fluent_builders::ListAnalyzers::set_type): <p>The type of analyzer.</p>
                            /// - On success, responds with [`ListAnalyzersOutput`](crate::output::ListAnalyzersOutput) with field(s):
    ///   - [`analyzers(Option<Vec<AnalyzerSummary>>)`](crate::output::ListAnalyzersOutput::analyzers): <p>The analyzers retrieved.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListAnalyzersOutput::next_token): <p>A token used for pagination of results returned.</p>
                            /// - On failure, responds with [`SdkError<ListAnalyzersError>`](crate::error::ListAnalyzersError)
    pub fn list_analyzers(&self) -> crate::client::fluent_builders::ListAnalyzers {
                                crate::client::fluent_builders::ListAnalyzers::new(self.handle.clone())
                            }
}

