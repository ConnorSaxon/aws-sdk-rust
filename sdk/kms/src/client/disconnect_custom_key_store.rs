// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisconnectCustomKeyStore`](crate::client::fluent_builders::DisconnectCustomKeyStore) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`custom_key_store_id(impl Into<String>)`](crate::client::fluent_builders::DisconnectCustomKeyStore::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::client::fluent_builders::DisconnectCustomKeyStore::set_custom_key_store_id): <p>Enter the ID of the custom key store you want to disconnect. To find the ID of a custom key store, use the <code>DescribeCustomKeyStores</code> operation.</p>
                            /// - On success, responds with [`DisconnectCustomKeyStoreOutput`](crate::output::DisconnectCustomKeyStoreOutput)
                            /// - On failure, responds with [`SdkError<DisconnectCustomKeyStoreError>`](crate::error::DisconnectCustomKeyStoreError)
    pub fn disconnect_custom_key_store(&self) -> crate::client::fluent_builders::DisconnectCustomKeyStore {
                                crate::client::fluent_builders::DisconnectCustomKeyStore::new(self.handle.clone())
                            }
}

