// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_journey_state_request(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::JourneyStateRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.state {
        object.key("State").string(var_1.as_str());
    }
    Ok(())
}

