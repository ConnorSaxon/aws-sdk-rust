// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disk_map(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DiskMap) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.original_disk_path {
        object.key("originalDiskPath").string(var_1.as_str());
    }
    if let Some(var_2) = &input.new_disk_name {
        object.key("newDiskName").string(var_2.as_str());
    }
    Ok(())
}

