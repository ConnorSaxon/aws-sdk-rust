// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyCacheSubnetGroup`](crate::client::fluent_builders::ModifyCacheSubnetGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cache_subnet_group_name(impl Into<String>)`](crate::client::fluent_builders::ModifyCacheSubnetGroup::cache_subnet_group_name) / [`set_cache_subnet_group_name(Option<String>)`](crate::client::fluent_builders::ModifyCacheSubnetGroup::set_cache_subnet_group_name): <p>The name for the cache subnet group. This value is stored as a lowercase string.</p>  <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p>  <p>Example: <code>mysubnetgroup</code> </p>
    ///   - [`cache_subnet_group_description(impl Into<String>)`](crate::client::fluent_builders::ModifyCacheSubnetGroup::cache_subnet_group_description) / [`set_cache_subnet_group_description(Option<String>)`](crate::client::fluent_builders::ModifyCacheSubnetGroup::set_cache_subnet_group_description): <p>A description of the cache subnet group.</p>
    ///   - [`subnet_ids(Vec<String>)`](crate::client::fluent_builders::ModifyCacheSubnetGroup::subnet_ids) / [`set_subnet_ids(Option<Vec<String>>)`](crate::client::fluent_builders::ModifyCacheSubnetGroup::set_subnet_ids): <p>The EC2 subnet IDs for the cache subnet group.</p>
                            /// - On success, responds with [`ModifyCacheSubnetGroupOutput`](crate::output::ModifyCacheSubnetGroupOutput) with field(s):
    ///   - [`cache_subnet_group(Option<CacheSubnetGroup>)`](crate::output::ModifyCacheSubnetGroupOutput::cache_subnet_group): <p>Represents the output of one of the following operations:</p>  <ul>   <li> <p> <code>CreateCacheSubnetGroup</code> </p> </li>   <li> <p> <code>ModifyCacheSubnetGroup</code> </p> </li>  </ul>
                            /// - On failure, responds with [`SdkError<ModifyCacheSubnetGroupError>`](crate::error::ModifyCacheSubnetGroupError)
    pub fn modify_cache_subnet_group(&self) -> crate::client::fluent_builders::ModifyCacheSubnetGroup {
                                crate::client::fluent_builders::ModifyCacheSubnetGroup::new(self.handle.clone())
                            }
}

