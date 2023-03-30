// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_evaluate_expression_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::EvaluateExpressionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pipeline_id {
        object.key("pipelineId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.object_id {
        object.key("objectId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.expression {
        object.key("expression").string(var_3.as_str());
    }
    Ok(())
}

