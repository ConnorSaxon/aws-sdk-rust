// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListExports`](crate::client::fluent_builders::ListExports) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListExports::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListExports::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListExports::set_next_token): <p>A string (provided by the <code>ListExports</code> response output) that identifies the next page of exported output values that you asked to retrieve.</p>
                            /// - On success, responds with [`ListExportsOutput`](crate::output::ListExportsOutput) with field(s):
    ///   - [`exports(Option<Vec<Export>>)`](crate::output::ListExportsOutput::exports): <p>The output for the <code>ListExports</code> action.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListExportsOutput::next_token): <p>If the output exceeds 100 exported output values, a string that identifies the next page of exports. If there is no additional page, this value is null.</p>
                            /// - On failure, responds with [`SdkError<ListExportsError>`](crate::error::ListExportsError)
    pub fn list_exports(&self) -> crate::client::fluent_builders::ListExports {
                                crate::client::fluent_builders::ListExports::new(self.handle.clone())
                            }
}

