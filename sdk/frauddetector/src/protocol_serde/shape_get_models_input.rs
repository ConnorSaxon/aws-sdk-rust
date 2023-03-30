// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_models_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetModelsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.model_id {
        object.key("modelId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.model_type {
        object.key("modelType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("nextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    Ok(())
}

