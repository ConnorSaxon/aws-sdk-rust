// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_accept_qualification_request_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AcceptQualificationRequestInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.qualification_request_id {
        object.key("QualificationRequestId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.integer_value {
        object.key("IntegerValue").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    Ok(())
}

