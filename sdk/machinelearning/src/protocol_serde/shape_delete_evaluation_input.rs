// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_evaluation_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteEvaluationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.evaluation_id {
        object.key("EvaluationId").string(var_1.as_str());
    }
    Ok(())
}

