// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CompleteLayerUpload`](crate::client::fluent_builders::CompleteLayerUpload) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`registry_id(impl Into<String>)`](crate::client::fluent_builders::CompleteLayerUpload::registry_id) / [`set_registry_id(Option<String>)`](crate::client::fluent_builders::CompleteLayerUpload::set_registry_id): <p>The Amazon Web Services account ID, or registry alias, associated with the registry where layers are uploaded. If you do not specify a registry, the default public registry is assumed.</p>
    ///   - [`repository_name(impl Into<String>)`](crate::client::fluent_builders::CompleteLayerUpload::repository_name) / [`set_repository_name(Option<String>)`](crate::client::fluent_builders::CompleteLayerUpload::set_repository_name): <p>The name of the repository in a public registry to associate with the image layer.</p>
    ///   - [`upload_id(impl Into<String>)`](crate::client::fluent_builders::CompleteLayerUpload::upload_id) / [`set_upload_id(Option<String>)`](crate::client::fluent_builders::CompleteLayerUpload::set_upload_id): <p>The upload ID from a previous <code>InitiateLayerUpload</code> operation to associate with the image layer.</p>
    ///   - [`layer_digests(Vec<String>)`](crate::client::fluent_builders::CompleteLayerUpload::layer_digests) / [`set_layer_digests(Option<Vec<String>>)`](crate::client::fluent_builders::CompleteLayerUpload::set_layer_digests): <p>The <code>sha256</code> digest of the image layer.</p>
                            /// - On success, responds with [`CompleteLayerUploadOutput`](crate::output::CompleteLayerUploadOutput) with field(s):
    ///   - [`registry_id(Option<String>)`](crate::output::CompleteLayerUploadOutput::registry_id): <p>The public registry ID that's associated with the request.</p>
    ///   - [`repository_name(Option<String>)`](crate::output::CompleteLayerUploadOutput::repository_name): <p>The repository name that's associated with the request.</p>
    ///   - [`upload_id(Option<String>)`](crate::output::CompleteLayerUploadOutput::upload_id): <p>The upload ID that's associated with the layer.</p>
    ///   - [`layer_digest(Option<String>)`](crate::output::CompleteLayerUploadOutput::layer_digest): <p>The <code>sha256</code> digest of the image layer.</p>
                            /// - On failure, responds with [`SdkError<CompleteLayerUploadError>`](crate::error::CompleteLayerUploadError)
    pub fn complete_layer_upload(&self) -> crate::client::fluent_builders::CompleteLayerUpload {
                                crate::client::fluent_builders::CompleteLayerUpload::new(self.handle.clone())
                            }
}

