// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListBots`](crate::client::fluent_builders::ListBots) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListBots::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::ListBots::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::ListBots::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListBots::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListBots::set_max_results): <p>The maximum number of results to return in a single call. The default is 10.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListBots::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListBots::set_next_token): <p>The token to use to retrieve the next page of results.</p>
                            /// - On success, responds with [`ListBotsOutput`](crate::output::ListBotsOutput) with field(s):
    ///   - [`bots(Option<Vec<Bot>>)`](crate::output::ListBotsOutput::bots): <p>List of bots and bot details.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListBotsOutput::next_token): <p>The token to use to retrieve the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListBotsError>`](crate::error::ListBotsError)
    pub fn list_bots(&self) -> crate::client::fluent_builders::ListBots {
                                crate::client::fluent_builders::ListBots::new(self.handle.clone())
                            }
}

