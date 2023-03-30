// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutExternalModel`](crate::client::fluent_builders::PutExternalModel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`model_endpoint(impl Into<String>)`](crate::client::fluent_builders::PutExternalModel::model_endpoint) / [`set_model_endpoint(Option<String>)`](crate::client::fluent_builders::PutExternalModel::set_model_endpoint): <p>The model endpoints name.</p>
    ///   - [`model_source(ModelSource)`](crate::client::fluent_builders::PutExternalModel::model_source) / [`set_model_source(Option<ModelSource>)`](crate::client::fluent_builders::PutExternalModel::set_model_source): <p>The source of the model.</p>
    ///   - [`invoke_model_endpoint_role_arn(impl Into<String>)`](crate::client::fluent_builders::PutExternalModel::invoke_model_endpoint_role_arn) / [`set_invoke_model_endpoint_role_arn(Option<String>)`](crate::client::fluent_builders::PutExternalModel::set_invoke_model_endpoint_role_arn): <p>The IAM role used to invoke the model endpoint.</p>
    ///   - [`input_configuration(ModelInputConfiguration)`](crate::client::fluent_builders::PutExternalModel::input_configuration) / [`set_input_configuration(Option<ModelInputConfiguration>)`](crate::client::fluent_builders::PutExternalModel::set_input_configuration): <p>The model endpoint input configuration.</p>
    ///   - [`output_configuration(ModelOutputConfiguration)`](crate::client::fluent_builders::PutExternalModel::output_configuration) / [`set_output_configuration(Option<ModelOutputConfiguration>)`](crate::client::fluent_builders::PutExternalModel::set_output_configuration): <p>The model endpoint output configuration.</p>
    ///   - [`model_endpoint_status(ModelEndpointStatus)`](crate::client::fluent_builders::PutExternalModel::model_endpoint_status) / [`set_model_endpoint_status(Option<ModelEndpointStatus>)`](crate::client::fluent_builders::PutExternalModel::set_model_endpoint_status): <p>The model endpoint’s status in Amazon Fraud Detector.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::PutExternalModel::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::PutExternalModel::set_tags): <p>A collection of key and value pairs.</p>
                            /// - On success, responds with [`PutExternalModelOutput`](crate::output::PutExternalModelOutput)
                            /// - On failure, responds with [`SdkError<PutExternalModelError>`](crate::error::PutExternalModelError)
    pub fn put_external_model(&self) -> crate::client::fluent_builders::PutExternalModel {
                                crate::client::fluent_builders::PutExternalModel::new(self.handle.clone())
                            }
}

