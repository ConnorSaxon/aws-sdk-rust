// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_dataset_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteDatasetGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.dataset_group_arn {
        object.key("datasetGroupArn").string(var_1.as_str());
    }
    Ok(())
}

