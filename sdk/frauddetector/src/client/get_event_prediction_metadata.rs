// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetEventPredictionMetadata`](crate::client::fluent_builders::GetEventPredictionMetadata) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`event_id(impl Into<String>)`](crate::client::fluent_builders::GetEventPredictionMetadata::event_id) / [`set_event_id(Option<String>)`](crate::client::fluent_builders::GetEventPredictionMetadata::set_event_id): <p> The event ID. </p>
    ///   - [`event_type_name(impl Into<String>)`](crate::client::fluent_builders::GetEventPredictionMetadata::event_type_name) / [`set_event_type_name(Option<String>)`](crate::client::fluent_builders::GetEventPredictionMetadata::set_event_type_name): <p> The event type associated with the detector specified for the prediction. </p>
    ///   - [`detector_id(impl Into<String>)`](crate::client::fluent_builders::GetEventPredictionMetadata::detector_id) / [`set_detector_id(Option<String>)`](crate::client::fluent_builders::GetEventPredictionMetadata::set_detector_id): <p> The detector ID. </p>
    ///   - [`detector_version_id(impl Into<String>)`](crate::client::fluent_builders::GetEventPredictionMetadata::detector_version_id) / [`set_detector_version_id(Option<String>)`](crate::client::fluent_builders::GetEventPredictionMetadata::set_detector_version_id): <p> The detector version ID. </p>
    ///   - [`prediction_timestamp(impl Into<String>)`](crate::client::fluent_builders::GetEventPredictionMetadata::prediction_timestamp) / [`set_prediction_timestamp(Option<String>)`](crate::client::fluent_builders::GetEventPredictionMetadata::set_prediction_timestamp): <p> The timestamp that defines when the prediction was generated. The timestamp must be specified using ISO 8601 standard in UTC.</p>  <p>We recommend calling <a href="https://docs.aws.amazon.com/frauddetector/latest/api/API_ListEventPredictions.html">ListEventPredictions</a> first, and using the <code>predictionTimestamp</code> value in the response to provide an accurate prediction timestamp value.</p>
                            /// - On success, responds with [`GetEventPredictionMetadataOutput`](crate::output::GetEventPredictionMetadataOutput) with field(s):
    ///   - [`event_id(Option<String>)`](crate::output::GetEventPredictionMetadataOutput::event_id): <p> The event ID. </p>
    ///   - [`event_type_name(Option<String>)`](crate::output::GetEventPredictionMetadataOutput::event_type_name): <p> The event type associated with the detector specified for this prediction. </p>
    ///   - [`entity_id(Option<String>)`](crate::output::GetEventPredictionMetadataOutput::entity_id): <p> The entity ID. </p>
    ///   - [`entity_type(Option<String>)`](crate::output::GetEventPredictionMetadataOutput::entity_type): <p> The entity type. </p>
    ///   - [`event_timestamp(Option<String>)`](crate::output::GetEventPredictionMetadataOutput::event_timestamp): <p> The timestamp for when the prediction was generated for the associated event ID. </p>
    ///   - [`detector_id(Option<String>)`](crate::output::GetEventPredictionMetadataOutput::detector_id): <p> The detector ID. </p>
    ///   - [`detector_version_id(Option<String>)`](crate::output::GetEventPredictionMetadataOutput::detector_version_id): <p> The detector version ID. </p>
    ///   - [`detector_version_status(Option<String>)`](crate::output::GetEventPredictionMetadataOutput::detector_version_status): <p> The status of the detector version. </p>
    ///   - [`event_variables(Option<Vec<EventVariableSummary>>)`](crate::output::GetEventPredictionMetadataOutput::event_variables): <p> A list of event variables that influenced the prediction scores. </p>
    ///   - [`rules(Option<Vec<EvaluatedRule>>)`](crate::output::GetEventPredictionMetadataOutput::rules): <p> List of rules associated with the detector version that were used for evaluating variable values. </p>
    ///   - [`rule_execution_mode(Option<RuleExecutionMode>)`](crate::output::GetEventPredictionMetadataOutput::rule_execution_mode): <p> The execution mode of the rule used for evaluating variable values. </p>
    ///   - [`outcomes(Option<Vec<String>>)`](crate::output::GetEventPredictionMetadataOutput::outcomes): <p> The outcomes of the matched rule, based on the rule execution mode. </p>
    ///   - [`evaluated_model_versions(Option<Vec<EvaluatedModelVersion>>)`](crate::output::GetEventPredictionMetadataOutput::evaluated_model_versions): <p> Model versions that were evaluated for generating predictions. </p>
    ///   - [`evaluated_external_models(Option<Vec<EvaluatedExternalModel>>)`](crate::output::GetEventPredictionMetadataOutput::evaluated_external_models): <p> External (Amazon SageMaker) models that were evaluated for generating predictions. </p>
    ///   - [`prediction_timestamp(Option<String>)`](crate::output::GetEventPredictionMetadataOutput::prediction_timestamp): <p>The timestamp that defines when the prediction was generated. </p>
                            /// - On failure, responds with [`SdkError<GetEventPredictionMetadataError>`](crate::error::GetEventPredictionMetadataError)
    pub fn get_event_prediction_metadata(&self) -> crate::client::fluent_builders::GetEventPredictionMetadata {
                                crate::client::fluent_builders::GetEventPredictionMetadata::new(self.handle.clone())
                            }
}

