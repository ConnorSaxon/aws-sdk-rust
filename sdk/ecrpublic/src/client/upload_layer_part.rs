// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UploadLayerPart`](crate::client::fluent_builders::UploadLayerPart) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`registry_id(impl Into<String>)`](crate::client::fluent_builders::UploadLayerPart::registry_id) / [`set_registry_id(Option<String>)`](crate::client::fluent_builders::UploadLayerPart::set_registry_id): <p>The Amazon Web Services account ID, or registry alias, that's associated with the registry that you're uploading layer parts to. If you do not specify a registry, the default public registry is assumed.</p>
    ///   - [`repository_name(impl Into<String>)`](crate::client::fluent_builders::UploadLayerPart::repository_name) / [`set_repository_name(Option<String>)`](crate::client::fluent_builders::UploadLayerPart::set_repository_name): <p>The name of the repository that you're uploading layer parts to.</p>
    ///   - [`upload_id(impl Into<String>)`](crate::client::fluent_builders::UploadLayerPart::upload_id) / [`set_upload_id(Option<String>)`](crate::client::fluent_builders::UploadLayerPart::set_upload_id): <p>The upload ID from a previous <code>InitiateLayerUpload</code> operation to associate with the layer part upload.</p>
    ///   - [`part_first_byte(i64)`](crate::client::fluent_builders::UploadLayerPart::part_first_byte) / [`set_part_first_byte(Option<i64>)`](crate::client::fluent_builders::UploadLayerPart::set_part_first_byte): <p>The position of the first byte of the layer part witin the overall image layer.</p>
    ///   - [`part_last_byte(i64)`](crate::client::fluent_builders::UploadLayerPart::part_last_byte) / [`set_part_last_byte(Option<i64>)`](crate::client::fluent_builders::UploadLayerPart::set_part_last_byte): <p>The position of the last byte of the layer part within the overall image layer.</p>
    ///   - [`layer_part_blob(Blob)`](crate::client::fluent_builders::UploadLayerPart::layer_part_blob) / [`set_layer_part_blob(Option<Blob>)`](crate::client::fluent_builders::UploadLayerPart::set_layer_part_blob): <p>The base64-encoded layer part payload.</p>
                            /// - On success, responds with [`UploadLayerPartOutput`](crate::output::UploadLayerPartOutput) with field(s):
    ///   - [`registry_id(Option<String>)`](crate::output::UploadLayerPartOutput::registry_id): <p>The registry ID that's associated with the request.</p>
    ///   - [`repository_name(Option<String>)`](crate::output::UploadLayerPartOutput::repository_name): <p>The repository name that's associated with the request.</p>
    ///   - [`upload_id(Option<String>)`](crate::output::UploadLayerPartOutput::upload_id): <p>The upload ID that's associated with the request.</p>
    ///   - [`last_byte_received(Option<i64>)`](crate::output::UploadLayerPartOutput::last_byte_received): <p>The integer value of the last byte that's received in the request.</p>
                            /// - On failure, responds with [`SdkError<UploadLayerPartError>`](crate::error::UploadLayerPartError)
    pub fn upload_layer_part(&self) -> crate::client::fluent_builders::UploadLayerPart {
                                crate::client::fluent_builders::UploadLayerPart::new(self.handle.clone())
                            }
}

