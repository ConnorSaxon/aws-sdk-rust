// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_created_artifacts_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListCreatedArtifactsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.progress_update_stream {
        object.key("ProgressUpdateStream").string(var_1.as_str());
    }
    if let Some(var_2) = &input.migration_task_name {
        object.key("MigrationTaskName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    Ok(())
}

