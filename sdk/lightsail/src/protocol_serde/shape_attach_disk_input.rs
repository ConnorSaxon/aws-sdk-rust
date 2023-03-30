// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_attach_disk_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AttachDiskInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.disk_name {
        object.key("diskName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.instance_name {
        object.key("instanceName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.disk_path {
        object.key("diskPath").string(var_3.as_str());
    }
    Ok(())
}

