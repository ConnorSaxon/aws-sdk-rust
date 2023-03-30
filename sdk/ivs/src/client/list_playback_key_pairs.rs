// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPlaybackKeyPairs`](crate::client::fluent_builders::ListPlaybackKeyPairs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPlaybackKeyPairs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPlaybackKeyPairs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPlaybackKeyPairs::set_next_token): <p>The first key pair to retrieve. This is used for pagination; see the <code>nextToken</code> response field.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPlaybackKeyPairs::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListPlaybackKeyPairs::set_max_results): <p>Maximum number of key pairs to return. Default: your service quota or 100, whichever is smaller.</p>
                            /// - On success, responds with [`ListPlaybackKeyPairsOutput`](crate::output::ListPlaybackKeyPairsOutput) with field(s):
    ///   - [`key_pairs(Option<Vec<PlaybackKeyPairSummary>>)`](crate::output::ListPlaybackKeyPairsOutput::key_pairs): <p>List of key pairs.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPlaybackKeyPairsOutput::next_token): <p>If there are more key pairs than <code>maxResults</code>, use <code>nextToken</code> in the request to get the next set.</p>
                            /// - On failure, responds with [`SdkError<ListPlaybackKeyPairsError>`](crate::error::ListPlaybackKeyPairsError)
    pub fn list_playback_key_pairs(&self) -> crate::client::fluent_builders::ListPlaybackKeyPairs {
                                crate::client::fluent_builders::ListPlaybackKeyPairs::new(self.handle.clone())
                            }
}

