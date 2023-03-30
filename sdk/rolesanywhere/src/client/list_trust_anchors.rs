// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTrustAnchors`](crate::client::fluent_builders::ListTrustAnchors) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListTrustAnchors::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListTrustAnchors::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListTrustAnchors::set_next_token): <p>A token that indicates where the output should continue from, if a previous operation did not show all results. To get the next results, call the operation again with this value.</p>
    ///   - [`page_size(i32)`](crate::client::fluent_builders::ListTrustAnchors::page_size) / [`set_page_size(Option<i32>)`](crate::client::fluent_builders::ListTrustAnchors::set_page_size): <p>The number of resources in the paginated list. </p>
                            /// - On success, responds with [`ListTrustAnchorsOutput`](crate::output::ListTrustAnchorsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListTrustAnchorsOutput::next_token): <p>A token that indicates where the output should continue from, if a previous operation did not show all results. To get the next results, call the operation again with this value.</p>
    ///   - [`trust_anchors(Option<Vec<TrustAnchorDetail>>)`](crate::output::ListTrustAnchorsOutput::trust_anchors): <p>A list of trust anchors.</p>
                            /// - On failure, responds with [`SdkError<ListTrustAnchorsError>`](crate::error::ListTrustAnchorsError)
    pub fn list_trust_anchors(&self) -> crate::client::fluent_builders::ListTrustAnchors {
                                crate::client::fluent_builders::ListTrustAnchors::new(self.handle.clone())
                            }
}

