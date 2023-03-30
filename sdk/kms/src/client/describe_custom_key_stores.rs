// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeCustomKeyStores`](crate::client::fluent_builders::DescribeCustomKeyStores) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeCustomKeyStores::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`custom_key_store_id(impl Into<String>)`](crate::client::fluent_builders::DescribeCustomKeyStores::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::client::fluent_builders::DescribeCustomKeyStores::set_custom_key_store_id): <p>Gets only information about the specified custom key store. Enter the key store ID.</p>  <p>By default, this operation gets information about all custom key stores in the account and Region. To limit the output to a particular custom key store, provide either the <code>CustomKeyStoreId</code> or <code>CustomKeyStoreName</code> parameter, but not both.</p>
    ///   - [`custom_key_store_name(impl Into<String>)`](crate::client::fluent_builders::DescribeCustomKeyStores::custom_key_store_name) / [`set_custom_key_store_name(Option<String>)`](crate::client::fluent_builders::DescribeCustomKeyStores::set_custom_key_store_name): <p>Gets only information about the specified custom key store. Enter the friendly name of the custom key store.</p>  <p>By default, this operation gets information about all custom key stores in the account and Region. To limit the output to a particular custom key store, provide either the <code>CustomKeyStoreId</code> or <code>CustomKeyStoreName</code> parameter, but not both.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::DescribeCustomKeyStores::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::DescribeCustomKeyStores::set_limit): <p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeCustomKeyStores::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeCustomKeyStores::set_marker): <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
                            /// - On success, responds with [`DescribeCustomKeyStoresOutput`](crate::output::DescribeCustomKeyStoresOutput) with field(s):
    ///   - [`custom_key_stores(Option<Vec<CustomKeyStoresListEntry>>)`](crate::output::DescribeCustomKeyStoresOutput::custom_key_stores): <p>Contains metadata about each custom key store.</p>
    ///   - [`next_marker(Option<String>)`](crate::output::DescribeCustomKeyStoresOutput::next_marker): <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    ///   - [`truncated(bool)`](crate::output::DescribeCustomKeyStoresOutput::truncated): <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
                            /// - On failure, responds with [`SdkError<DescribeCustomKeyStoresError>`](crate::error::DescribeCustomKeyStoresError)
    pub fn describe_custom_key_stores(&self) -> crate::client::fluent_builders::DescribeCustomKeyStores {
                                crate::client::fluent_builders::DescribeCustomKeyStores::new(self.handle.clone())
                            }
}

