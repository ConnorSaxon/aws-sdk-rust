// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_data_quality_result_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetDataQualityResultInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.result_id {
        object.key("ResultId").string(var_1.as_str());
    }
    Ok(())
}

