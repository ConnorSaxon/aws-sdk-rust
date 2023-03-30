// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateApiCache`](crate::client::fluent_builders::CreateApiCache) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`api_id(impl Into<String>)`](crate::client::fluent_builders::CreateApiCache::api_id) / [`set_api_id(Option<String>)`](crate::client::fluent_builders::CreateApiCache::set_api_id): <p>The GraphQL API ID.</p>
    ///   - [`ttl(i64)`](crate::client::fluent_builders::CreateApiCache::ttl) / [`set_ttl(i64)`](crate::client::fluent_builders::CreateApiCache::set_ttl): <p>TTL in seconds for cache entries.</p>  <p>Valid values are 1–3,600 seconds.</p>
    ///   - [`transit_encryption_enabled(bool)`](crate::client::fluent_builders::CreateApiCache::transit_encryption_enabled) / [`set_transit_encryption_enabled(bool)`](crate::client::fluent_builders::CreateApiCache::set_transit_encryption_enabled): <p>Transit encryption flag when connecting to cache. You cannot update this setting after creation.</p>
    ///   - [`at_rest_encryption_enabled(bool)`](crate::client::fluent_builders::CreateApiCache::at_rest_encryption_enabled) / [`set_at_rest_encryption_enabled(bool)`](crate::client::fluent_builders::CreateApiCache::set_at_rest_encryption_enabled): <p>At-rest encryption flag for cache. You cannot update this setting after creation.</p>
    ///   - [`api_caching_behavior(ApiCachingBehavior)`](crate::client::fluent_builders::CreateApiCache::api_caching_behavior) / [`set_api_caching_behavior(Option<ApiCachingBehavior>)`](crate::client::fluent_builders::CreateApiCache::set_api_caching_behavior): <p>Caching behavior.</p>  <ul>   <li> <p> <b>FULL_REQUEST_CACHING</b>: All requests are fully cached.</p> </li>   <li> <p> <b>PER_RESOLVER_CACHING</b>: Individual resolvers that you specify are cached.</p> </li>  </ul>
    ///   - [`r#type(ApiCacheType)`](crate::client::fluent_builders::CreateApiCache::type) / [`set_type(Option<ApiCacheType>)`](crate::client::fluent_builders::CreateApiCache::set_type): <p>The cache instance type. Valid values are </p>  <ul>   <li> <p> <code>SMALL</code> </p> </li>   <li> <p> <code>MEDIUM</code> </p> </li>   <li> <p> <code>LARGE</code> </p> </li>   <li> <p> <code>XLARGE</code> </p> </li>   <li> <p> <code>LARGE_2X</code> </p> </li>   <li> <p> <code>LARGE_4X</code> </p> </li>   <li> <p> <code>LARGE_8X</code> (not available in all regions)</p> </li>   <li> <p> <code>LARGE_12X</code> </p> </li>  </ul>  <p>Historically, instance types were identified by an EC2-style value. As of July 2020, this is deprecated, and the generic identifiers above should be used.</p>  <p>The following legacy instance types are available, but their use is discouraged:</p>  <ul>   <li> <p> <b>T2_SMALL</b>: A t2.small instance type.</p> </li>   <li> <p> <b>T2_MEDIUM</b>: A t2.medium instance type.</p> </li>   <li> <p> <b>R4_LARGE</b>: A r4.large instance type.</p> </li>   <li> <p> <b>R4_XLARGE</b>: A r4.xlarge instance type.</p> </li>   <li> <p> <b>R4_2XLARGE</b>: A r4.2xlarge instance type.</p> </li>   <li> <p> <b>R4_4XLARGE</b>: A r4.4xlarge instance type.</p> </li>   <li> <p> <b>R4_8XLARGE</b>: A r4.8xlarge instance type.</p> </li>  </ul>
                            /// - On success, responds with [`CreateApiCacheOutput`](crate::output::CreateApiCacheOutput) with field(s):
    ///   - [`api_cache(Option<ApiCache>)`](crate::output::CreateApiCacheOutput::api_cache): <p>The <code>ApiCache</code> object.</p>
                            /// - On failure, responds with [`SdkError<CreateApiCacheError>`](crate::error::CreateApiCacheError)
    pub fn create_api_cache(&self) -> crate::client::fluent_builders::CreateApiCache {
                                crate::client::fluent_builders::CreateApiCache::new(self.handle.clone())
                            }
}

