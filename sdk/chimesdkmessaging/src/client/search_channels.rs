// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SearchChannels`](crate::client::fluent_builders::SearchChannels) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::SearchChannels::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`chime_bearer(impl Into<String>)`](crate::client::fluent_builders::SearchChannels::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::client::fluent_builders::SearchChannels::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    ///   - [`fields(Vec<SearchField>)`](crate::client::fluent_builders::SearchChannels::fields) / [`set_fields(Option<Vec<SearchField>>)`](crate::client::fluent_builders::SearchChannels::set_fields): <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::SearchChannels::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::SearchChannels::set_max_results): <p>The maximum number of channels that you want returned.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::SearchChannels::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::SearchChannels::set_next_token): <p>The token returned from previous API requests until the number of channels is reached.</p>
                            /// - On success, responds with [`SearchChannelsOutput`](crate::output::SearchChannelsOutput) with field(s):
    ///   - [`channels(Option<Vec<ChannelSummary>>)`](crate::output::SearchChannelsOutput::channels): <p>A list of the channels in the request.</p>
    ///   - [`next_token(Option<String>)`](crate::output::SearchChannelsOutput::next_token): <p>The token returned from previous API responses until the number of channels is reached.</p>
                            /// - On failure, responds with [`SdkError<SearchChannelsError>`](crate::error::SearchChannelsError)
    pub fn search_channels(&self) -> crate::client::fluent_builders::SearchChannels {
                                crate::client::fluent_builders::SearchChannels::new(self.handle.clone())
                            }
}

