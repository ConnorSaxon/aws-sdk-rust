// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_export_tasks_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeExportTasksInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.task_id {
        object.key("taskId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.status_code {
        object.key("statusCode").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("nextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    Ok(())
}

