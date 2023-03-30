// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_event_prediction_metadata_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetEventPredictionMetadataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_id {
        object.key("eventId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.event_type_name {
        object.key("eventTypeName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.detector_id {
        object.key("detectorId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.detector_version_id {
        object.key("detectorVersionId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.prediction_timestamp {
        object.key("predictionTimestamp").string(var_5.as_str());
    }
    Ok(())
}

