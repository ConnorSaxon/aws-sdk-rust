// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_task_run_sort_criteria(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TaskRunSortCriteria) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.column {
        object.key("Column").string(var_1.as_str());
    }
    if let Some(var_2) = &input.sort_direction {
        object.key("SortDirection").string(var_2.as_str());
    }
    Ok(())
}

