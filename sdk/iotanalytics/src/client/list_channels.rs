// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListChannels`](crate::client::fluent_builders::ListChannels) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListChannels::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListChannels::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListChannels::set_next_token): <p>The token for the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListChannels::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListChannels::set_max_results): <p>The maximum number of results to return in this request.</p>  <p>The default value is 100.</p>
                            /// - On success, responds with [`ListChannelsOutput`](crate::output::ListChannelsOutput) with field(s):
    ///   - [`channel_summaries(Option<Vec<ChannelSummary>>)`](crate::output::ListChannelsOutput::channel_summaries): <p>A list of <code>ChannelSummary</code> objects.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListChannelsOutput::next_token): <p>The token to retrieve the next set of results, or <code>null</code> if there are no more results.</p>
                            /// - On failure, responds with [`SdkError<ListChannelsError>`](crate::error::ListChannelsError)
    pub fn list_channels(&self) -> crate::client::fluent_builders::ListChannels {
                                crate::client::fluent_builders::ListChannels::new(self.handle.clone())
                            }
}

