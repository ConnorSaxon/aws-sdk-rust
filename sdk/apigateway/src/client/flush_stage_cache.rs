// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`FlushStageCache`](crate::client::fluent_builders::FlushStageCache) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl Into<String>)`](crate::client::fluent_builders::FlushStageCache::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::client::fluent_builders::FlushStageCache::set_rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`stage_name(impl Into<String>)`](crate::client::fluent_builders::FlushStageCache::stage_name) / [`set_stage_name(Option<String>)`](crate::client::fluent_builders::FlushStageCache::set_stage_name): <p>The name of the stage to flush its cache.</p>
                            /// - On success, responds with [`FlushStageCacheOutput`](crate::output::FlushStageCacheOutput)
                            /// - On failure, responds with [`SdkError<FlushStageCacheError>`](crate::error::FlushStageCacheError)
    pub fn flush_stage_cache(&self) -> crate::client::fluent_builders::FlushStageCache {
                                crate::client::fluent_builders::FlushStageCache::new(self.handle.clone())
                            }
}

