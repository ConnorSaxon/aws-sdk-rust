// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDevEndpoints`](crate::client::fluent_builders::GetDevEndpoints) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetDevEndpoints::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetDevEndpoints::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetDevEndpoints::set_max_results): <p>The maximum size of information to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetDevEndpoints::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetDevEndpoints::set_next_token): <p>A continuation token, if this is a continuation call.</p>
                            /// - On success, responds with [`GetDevEndpointsOutput`](crate::output::GetDevEndpointsOutput) with field(s):
    ///   - [`dev_endpoints(Option<Vec<DevEndpoint>>)`](crate::output::GetDevEndpointsOutput::dev_endpoints): <p>A list of <code>DevEndpoint</code> definitions.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetDevEndpointsOutput::next_token): <p>A continuation token, if not all <code>DevEndpoint</code> definitions have yet been returned.</p>
                            /// - On failure, responds with [`SdkError<GetDevEndpointsError>`](crate::error::GetDevEndpointsError)
    pub fn get_dev_endpoints(&self) -> crate::client::fluent_builders::GetDevEndpoints {
                                crate::client::fluent_builders::GetDevEndpoints::new(self.handle.clone())
                            }
}

