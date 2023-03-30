// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListByoipCidrs`](crate::client::fluent_builders::ListByoipCidrs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListByoipCidrs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListByoipCidrs::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListByoipCidrs::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListByoipCidrs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListByoipCidrs::set_next_token): <p>The token for the next page of results.</p>
                            /// - On success, responds with [`ListByoipCidrsOutput`](crate::output::ListByoipCidrsOutput) with field(s):
    ///   - [`byoip_cidrs(Option<Vec<ByoipCidr>>)`](crate::output::ListByoipCidrsOutput::byoip_cidrs): <p>Information about your address ranges.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListByoipCidrsOutput::next_token): <p>The token for the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListByoipCidrsError>`](crate::error::ListByoipCidrsError)
    pub fn list_byoip_cidrs(&self) -> crate::client::fluent_builders::ListByoipCidrs {
                                crate::client::fluent_builders::ListByoipCidrs::new(self.handle.clone())
                            }
}

