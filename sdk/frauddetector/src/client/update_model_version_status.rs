// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateModelVersionStatus`](crate::client::fluent_builders::UpdateModelVersionStatus) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`model_id(impl Into<String>)`](crate::client::fluent_builders::UpdateModelVersionStatus::model_id) / [`set_model_id(Option<String>)`](crate::client::fluent_builders::UpdateModelVersionStatus::set_model_id): <p>The model ID of the model version to update.</p>
    ///   - [`model_type(ModelTypeEnum)`](crate::client::fluent_builders::UpdateModelVersionStatus::model_type) / [`set_model_type(Option<ModelTypeEnum>)`](crate::client::fluent_builders::UpdateModelVersionStatus::set_model_type): <p>The model type.</p>
    ///   - [`model_version_number(impl Into<String>)`](crate::client::fluent_builders::UpdateModelVersionStatus::model_version_number) / [`set_model_version_number(Option<String>)`](crate::client::fluent_builders::UpdateModelVersionStatus::set_model_version_number): <p>The model version number.</p>
    ///   - [`status(ModelVersionStatus)`](crate::client::fluent_builders::UpdateModelVersionStatus::status) / [`set_status(Option<ModelVersionStatus>)`](crate::client::fluent_builders::UpdateModelVersionStatus::set_status): <p>The model version status.</p>
                            /// - On success, responds with [`UpdateModelVersionStatusOutput`](crate::output::UpdateModelVersionStatusOutput)
                            /// - On failure, responds with [`SdkError<UpdateModelVersionStatusError>`](crate::error::UpdateModelVersionStatusError)
    pub fn update_model_version_status(&self) -> crate::client::fluent_builders::UpdateModelVersionStatus {
                                crate::client::fluent_builders::UpdateModelVersionStatus::new(self.handle.clone())
                            }
}

