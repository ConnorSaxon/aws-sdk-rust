// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateLocationObjectStorage`](crate::client::fluent_builders::CreateLocationObjectStorage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`server_hostname(impl Into<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::server_hostname) / [`set_server_hostname(Option<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::set_server_hostname): <p>Specifies the domain name or IP address of the object storage server. A DataSync agent uses this hostname to mount the object storage server in a network.</p>
    ///   - [`server_port(i32)`](crate::client::fluent_builders::CreateLocationObjectStorage::server_port) / [`set_server_port(Option<i32>)`](crate::client::fluent_builders::CreateLocationObjectStorage::set_server_port): <p>Specifies the port that your object storage server accepts inbound network traffic on (for example, port 443).</p>
    ///   - [`server_protocol(ObjectStorageServerProtocol)`](crate::client::fluent_builders::CreateLocationObjectStorage::server_protocol) / [`set_server_protocol(Option<ObjectStorageServerProtocol>)`](crate::client::fluent_builders::CreateLocationObjectStorage::set_server_protocol): <p>Specifies the protocol that your object storage server uses to communicate.</p>
    ///   - [`subdirectory(impl Into<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::subdirectory) / [`set_subdirectory(Option<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::set_subdirectory): <p>Specifies the object prefix for your object storage server. If this is a source location, DataSync only copies objects with this prefix. If this is a destination location, DataSync writes all objects with this prefix. </p>
    ///   - [`bucket_name(impl Into<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::bucket_name) / [`set_bucket_name(Option<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::set_bucket_name): <p>Specifies the name of the object storage bucket involved in the transfer.</p>
    ///   - [`access_key(impl Into<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::access_key) / [`set_access_key(Option<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::set_access_key): <p>Specifies the access key (for example, a user name) if credentials are required to authenticate with the object storage server.</p>
    ///   - [`secret_key(impl Into<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::secret_key) / [`set_secret_key(Option<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::set_secret_key): <p>Specifies the secret key (for example, a password) if credentials are required to authenticate with the object storage server.</p>
    ///   - [`agent_arns(Vec<String>)`](crate::client::fluent_builders::CreateLocationObjectStorage::agent_arns) / [`set_agent_arns(Option<Vec<String>>)`](crate::client::fluent_builders::CreateLocationObjectStorage::set_agent_arns): <p>Specifies the Amazon Resource Names (ARNs) of the DataSync agents that can securely connect with your location.</p>
    ///   - [`tags(Vec<TagListEntry>)`](crate::client::fluent_builders::CreateLocationObjectStorage::tags) / [`set_tags(Option<Vec<TagListEntry>>)`](crate::client::fluent_builders::CreateLocationObjectStorage::set_tags): <p>Specifies the key-value pair that represents a tag that you want to add to the resource. Tags can help you manage, filter, and search for your resources. We recommend creating a name tag for your location.</p>
    ///   - [`server_certificate(Blob)`](crate::client::fluent_builders::CreateLocationObjectStorage::server_certificate) / [`set_server_certificate(Option<Blob>)`](crate::client::fluent_builders::CreateLocationObjectStorage::set_server_certificate): <p>Specifies a certificate to authenticate with an object storage system that uses a private or self-signed certificate authority (CA). You must specify a Base64-encoded <code>.pem</code> file (for example, <code>file:///home/user/.ssh/storage_sys_certificate.pem</code>). The certificate can be up to 32768 bytes (before Base64 encoding).</p>  <p>To use this parameter, configure <code>ServerProtocol</code> to <code>HTTPS</code>.</p>
                            /// - On success, responds with [`CreateLocationObjectStorageOutput`](crate::output::CreateLocationObjectStorageOutput) with field(s):
    ///   - [`location_arn(Option<String>)`](crate::output::CreateLocationObjectStorageOutput::location_arn): <p>Specifies the ARN of the object storage system location that you create.</p>
                            /// - On failure, responds with [`SdkError<CreateLocationObjectStorageError>`](crate::error::CreateLocationObjectStorageError)
    pub fn create_location_object_storage(&self) -> crate::client::fluent_builders::CreateLocationObjectStorage {
                                crate::client::fluent_builders::CreateLocationObjectStorage::new(self.handle.clone())
                            }
}

