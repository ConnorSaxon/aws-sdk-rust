// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateCustomKeyStore`](crate::client::fluent_builders::UpdateCustomKeyStore) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`custom_key_store_id(impl Into<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::set_custom_key_store_id): <p>Identifies the custom key store that you want to update. Enter the ID of the custom key store. To find the ID of a custom key store, use the <code>DescribeCustomKeyStores</code> operation.</p>
    ///   - [`new_custom_key_store_name(impl Into<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::new_custom_key_store_name) / [`set_new_custom_key_store_name(Option<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::set_new_custom_key_store_name): <p>Changes the friendly name of the custom key store to the value that you specify. The custom key store name must be unique in the Amazon Web Services account.</p>  <p>To change this value, an CloudHSM key store must be disconnected. An external key store can be connected or disconnected.</p>
    ///   - [`key_store_password(impl Into<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::key_store_password) / [`set_key_store_password(Option<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::set_key_store_password): <p>Enter the current password of the <code>kmsuser</code> crypto user (CU) in the CloudHSM cluster that is associated with the custom key store. This parameter is valid only for custom key stores with a <code>CustomKeyStoreType</code> of <code>AWS_CLOUDHSM</code>.</p>  <p>This parameter tells KMS the current password of the <code>kmsuser</code> crypto user (CU). It does not set or change the password of any users in the CloudHSM cluster.</p>  <p>To change this value, the CloudHSM key store must be disconnected.</p>
    ///   - [`cloud_hsm_cluster_id(impl Into<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::cloud_hsm_cluster_id) / [`set_cloud_hsm_cluster_id(Option<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::set_cloud_hsm_cluster_id): <p>Associates the custom key store with a related CloudHSM cluster. This parameter is valid only for custom key stores with a <code>CustomKeyStoreType</code> of <code>AWS_CLOUDHSM</code>.</p>  <p>Enter the cluster ID of the cluster that you used to create the custom key store or a cluster that shares a backup history and has the same cluster certificate as the original cluster. You cannot use this parameter to associate a custom key store with an unrelated cluster. In addition, the replacement cluster must <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">fulfill the requirements</a> for a cluster associated with a custom key store. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>  <p>To change this value, the CloudHSM key store must be disconnected.</p>
    ///   - [`xks_proxy_uri_endpoint(impl Into<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::xks_proxy_uri_endpoint) / [`set_xks_proxy_uri_endpoint(Option<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::set_xks_proxy_uri_endpoint): <p>Changes the URI endpoint that KMS uses to connect to your external key store proxy (XKS proxy). This parameter is valid only for custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p>  <p>For external key stores with an <code>XksProxyConnectivity</code> value of <code>PUBLIC_ENDPOINT</code>, the protocol must be HTTPS.</p>  <p>For external key stores with an <code>XksProxyConnectivity</code> value of <code>VPC_ENDPOINT_SERVICE</code>, specify <code>https://</code> followed by the private DNS name associated with the VPC endpoint service. Each external key store must use a different private DNS name.</p>  <p>The combined <code>XksProxyUriEndpoint</code> and <code>XksProxyUriPath</code> values must be unique in the Amazon Web Services account and Region.</p>  <p>To change this value, the external key store must be disconnected.</p>
    ///   - [`xks_proxy_uri_path(impl Into<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::xks_proxy_uri_path) / [`set_xks_proxy_uri_path(Option<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::set_xks_proxy_uri_path): <p>Changes the base path to the proxy APIs for this external key store. To find this value, see the documentation for your external key manager and external key store proxy (XKS proxy). This parameter is valid only for custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p>  <p>The value must start with <code>/</code> and must end with <code>/kms/xks/v1</code>, where <code>v1</code> represents the version of the KMS external key store proxy API. You can include an optional prefix between the required elements such as <code>/<i>example</i>/kms/xks/v1</code>.</p>  <p>The combined <code>XksProxyUriEndpoint</code> and <code>XksProxyUriPath</code> values must be unique in the Amazon Web Services account and Region.</p>  <p>You can change this value when the external key store is connected or disconnected.</p>
    ///   - [`xks_proxy_vpc_endpoint_service_name(impl Into<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::xks_proxy_vpc_endpoint_service_name) / [`set_xks_proxy_vpc_endpoint_service_name(Option<String>)`](crate::client::fluent_builders::UpdateCustomKeyStore::set_xks_proxy_vpc_endpoint_service_name): <p>Changes the name that KMS uses to identify the Amazon VPC endpoint service for your external key store proxy (XKS proxy). This parameter is valid when the <code>CustomKeyStoreType</code> is <code>EXTERNAL_KEY_STORE</code> and the <code>XksProxyConnectivity</code> is <code>VPC_ENDPOINT_SERVICE</code>.</p>  <p>To change this value, the external key store must be disconnected.</p>
    ///   - [`xks_proxy_authentication_credential(XksProxyAuthenticationCredentialType)`](crate::client::fluent_builders::UpdateCustomKeyStore::xks_proxy_authentication_credential) / [`set_xks_proxy_authentication_credential(Option<XksProxyAuthenticationCredentialType>)`](crate::client::fluent_builders::UpdateCustomKeyStore::set_xks_proxy_authentication_credential): <p>Changes the credentials that KMS uses to sign requests to the external key store proxy (XKS proxy). This parameter is valid only for custom key stores with a <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p>  <p>You must specify both the <code>AccessKeyId</code> and <code>SecretAccessKey</code> value in the authentication credential, even if you are only updating one value.</p>  <p>This parameter doesn't establish or change your authentication credentials on the proxy. It just tells KMS the credential that you established with your external key store proxy. For example, if you rotate the credential on your external key store proxy, you can use this parameter to update the credential in KMS.</p>  <p>You can change this value when the external key store is connected or disconnected.</p>
    ///   - [`xks_proxy_connectivity(XksProxyConnectivityType)`](crate::client::fluent_builders::UpdateCustomKeyStore::xks_proxy_connectivity) / [`set_xks_proxy_connectivity(Option<XksProxyConnectivityType>)`](crate::client::fluent_builders::UpdateCustomKeyStore::set_xks_proxy_connectivity): <p>Changes the connectivity setting for the external key store. To indicate that the external key store proxy uses a Amazon VPC endpoint service to communicate with KMS, specify <code>VPC_ENDPOINT_SERVICE</code>. Otherwise, specify <code>PUBLIC_ENDPOINT</code>.</p>  <p>If you change the <code>XksProxyConnectivity</code> to <code>VPC_ENDPOINT_SERVICE</code>, you must also change the <code>XksProxyUriEndpoint</code> and add an <code>XksProxyVpcEndpointServiceName</code> value. </p>  <p>If you change the <code>XksProxyConnectivity</code> to <code>PUBLIC_ENDPOINT</code>, you must also change the <code>XksProxyUriEndpoint</code> and specify a null or empty string for the <code>XksProxyVpcEndpointServiceName</code> value.</p>  <p>To change this value, the external key store must be disconnected.</p>
                            /// - On success, responds with [`UpdateCustomKeyStoreOutput`](crate::output::UpdateCustomKeyStoreOutput)
                            /// - On failure, responds with [`SdkError<UpdateCustomKeyStoreError>`](crate::error::UpdateCustomKeyStoreError)
    pub fn update_custom_key_store(&self) -> crate::client::fluent_builders::UpdateCustomKeyStore {
                                crate::client::fluent_builders::UpdateCustomKeyStore::new(self.handle.clone())
                            }
}

