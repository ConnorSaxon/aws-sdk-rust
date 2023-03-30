// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteStage`](crate::client::fluent_builders::DeleteStage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl Into<String>)`](crate::client::fluent_builders::DeleteStage::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::client::fluent_builders::DeleteStage::set_rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`stage_name(impl Into<String>)`](crate::client::fluent_builders::DeleteStage::stage_name) / [`set_stage_name(Option<String>)`](crate::client::fluent_builders::DeleteStage::set_stage_name): <p>The name of the Stage resource to delete.</p>
                            /// - On success, responds with [`DeleteStageOutput`](crate::output::DeleteStageOutput)
                            /// - On failure, responds with [`SdkError<DeleteStageError>`](crate::error::DeleteStageError)
    pub fn delete_stage(&self) -> crate::client::fluent_builders::DeleteStage {
                                crate::client::fluent_builders::DeleteStage::new(self.handle.clone())
                            }
}

