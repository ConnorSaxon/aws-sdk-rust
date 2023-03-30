// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListKeys`](crate::client::fluent_builders::ListKeys) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListKeys::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListKeys::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::ListKeys::set_limit): <p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p>  <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListKeys::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListKeys::set_marker): <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
                            /// - On success, responds with [`ListKeysOutput`](crate::output::ListKeysOutput) with field(s):
    ///   - [`keys(Option<Vec<KeyListEntry>>)`](crate::output::ListKeysOutput::keys): <p>A list of KMS keys.</p>
    ///   - [`next_marker(Option<String>)`](crate::output::ListKeysOutput::next_marker): <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    ///   - [`truncated(bool)`](crate::output::ListKeysOutput::truncated): <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
                            /// - On failure, responds with [`SdkError<ListKeysError>`](crate::error::ListKeysError)
    pub fn list_keys(&self) -> crate::client::fluent_builders::ListKeys {
                                crate::client::fluent_builders::ListKeys::new(self.handle.clone())
                            }
}

