// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateFileCache`](crate::client::fluent_builders::CreateFileCache) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CreateFileCache::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CreateFileCache::set_client_request_token): <p>An idempotency token for resource creation, in a string of up to 64 ASCII characters. This token is automatically filled on your behalf when you use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p>  <p>By using the idempotent operation, you can retry a <code>CreateFileCache</code> operation without the risk of creating an extra cache. This approach can be useful when an initial call fails in a way that makes it unclear whether a cache was created. Examples are if a transport level timeout occurred, or your connection was reset. If you use the same client request token and the initial call created a cache, the client receives success as long as the parameters are the same.</p>
    ///   - [`file_cache_type(FileCacheType)`](crate::client::fluent_builders::CreateFileCache::file_cache_type) / [`set_file_cache_type(Option<FileCacheType>)`](crate::client::fluent_builders::CreateFileCache::set_file_cache_type): <p>The type of cache that you're creating, which must be <code>LUSTRE</code>.</p>
    ///   - [`file_cache_type_version(impl Into<String>)`](crate::client::fluent_builders::CreateFileCache::file_cache_type_version) / [`set_file_cache_type_version(Option<String>)`](crate::client::fluent_builders::CreateFileCache::set_file_cache_type_version): <p>Sets the Lustre version for the cache that you're creating, which must be <code>2.12</code>.</p>
    ///   - [`storage_capacity(i32)`](crate::client::fluent_builders::CreateFileCache::storage_capacity) / [`set_storage_capacity(Option<i32>)`](crate::client::fluent_builders::CreateFileCache::set_storage_capacity): <p>The storage capacity of the cache in gibibytes (GiB). Valid values are 1200 GiB, 2400 GiB, and increments of 2400 GiB.</p>
    ///   - [`subnet_ids(Vec<String>)`](crate::client::fluent_builders::CreateFileCache::subnet_ids) / [`set_subnet_ids(Option<Vec<String>>)`](crate::client::fluent_builders::CreateFileCache::set_subnet_ids): <p>A list of subnet IDs that the cache will be accessible from. You can specify only one subnet ID in a call to the <code>CreateFileCache</code> operation.</p>
    ///   - [`security_group_ids(Vec<String>)`](crate::client::fluent_builders::CreateFileCache::security_group_ids) / [`set_security_group_ids(Option<Vec<String>>)`](crate::client::fluent_builders::CreateFileCache::set_security_group_ids): <p>A list of IDs specifying the security groups to apply to all network interfaces created for Amazon File Cache access. This list isn't returned in later requests to describe the cache.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateFileCache::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateFileCache::set_tags): <p>A list of <code>Tag</code> values, with a maximum of 50 elements.</p>
    ///   - [`copy_tags_to_data_repository_associations(bool)`](crate::client::fluent_builders::CreateFileCache::copy_tags_to_data_repository_associations) / [`set_copy_tags_to_data_repository_associations(Option<bool>)`](crate::client::fluent_builders::CreateFileCache::set_copy_tags_to_data_repository_associations): <p>A boolean flag indicating whether tags for the cache should be copied to data repository associations. This value defaults to false.</p>
    ///   - [`kms_key_id(impl Into<String>)`](crate::client::fluent_builders::CreateFileCache::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::client::fluent_builders::CreateFileCache::set_kms_key_id): <p>Specifies the ID of the Key Management Service (KMS) key to use for encrypting data on an Amazon File Cache. If a <code>KmsKeyId</code> isn't specified, the Amazon FSx-managed KMS key for your account is used. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_Encrypt.html">Encrypt</a> in the <i>Key Management Service API Reference</i>.</p>
    ///   - [`lustre_configuration(CreateFileCacheLustreConfiguration)`](crate::client::fluent_builders::CreateFileCache::lustre_configuration) / [`set_lustre_configuration(Option<CreateFileCacheLustreConfiguration>)`](crate::client::fluent_builders::CreateFileCache::set_lustre_configuration): <p>The configuration for the Amazon File Cache resource being created.</p>
    ///   - [`data_repository_associations(Vec<FileCacheDataRepositoryAssociation>)`](crate::client::fluent_builders::CreateFileCache::data_repository_associations) / [`set_data_repository_associations(Option<Vec<FileCacheDataRepositoryAssociation>>)`](crate::client::fluent_builders::CreateFileCache::set_data_repository_associations): <p>A list of up to 8 configurations for data repository associations (DRAs) to be created during the cache creation. The DRAs link the cache to either an Amazon S3 data repository or a Network File System (NFS) data repository that supports the NFSv3 protocol.</p>  <p>The DRA configurations must meet the following requirements:</p>  <ul>   <li> <p>All configurations on the list must be of the same data repository type, either all S3 or all NFS. A cache can't link to different data repository types at the same time.</p> </li>   <li> <p>An NFS DRA must link to an NFS file system that supports the NFSv3 protocol.</p> </li>  </ul>  <p>DRA automatic import and automatic export is not supported.</p>
                            /// - On success, responds with [`CreateFileCacheOutput`](crate::output::CreateFileCacheOutput) with field(s):
    ///   - [`file_cache(Option<FileCacheCreating>)`](crate::output::CreateFileCacheOutput::file_cache): <p>A description of the cache that was created.</p>
                            /// - On failure, responds with [`SdkError<CreateFileCacheError>`](crate::error::CreateFileCacheError)
    pub fn create_file_cache(&self) -> crate::client::fluent_builders::CreateFileCache {
                                crate::client::fluent_builders::CreateFileCache::new(self.handle.clone())
                            }
}

