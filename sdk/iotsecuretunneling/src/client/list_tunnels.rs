// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTunnels`](crate::client::fluent_builders::ListTunnels) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListTunnels::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`thing_name(impl Into<String>)`](crate::client::fluent_builders::ListTunnels::thing_name) / [`set_thing_name(Option<String>)`](crate::client::fluent_builders::ListTunnels::set_thing_name): <p>The name of the IoT thing associated with the destination device.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListTunnels::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListTunnels::set_max_results): <p>The maximum number of results to return at once.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListTunnels::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListTunnels::set_next_token): <p>To retrieve the next set of results, the nextToken value from a previous response; otherwise null to receive the first set of results.</p>
                            /// - On success, responds with [`ListTunnelsOutput`](crate::output::ListTunnelsOutput) with field(s):
    ///   - [`tunnel_summaries(Option<Vec<TunnelSummary>>)`](crate::output::ListTunnelsOutput::tunnel_summaries): <p>A short description of the tunnels in an Amazon Web Services account.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListTunnelsOutput::next_token): <p>The token to use to get the next set of results, or null if there are no additional results.</p>
                            /// - On failure, responds with [`SdkError<ListTunnelsError>`](crate::error::ListTunnelsError)
    pub fn list_tunnels(&self) -> crate::client::fluent_builders::ListTunnels {
                                crate::client::fluent_builders::ListTunnels::new(self.handle.clone())
                            }
}

