// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_disks_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetDisksInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.page_token {
        object.key("pageToken").string(var_1.as_str());
    }
    Ok(())
}

