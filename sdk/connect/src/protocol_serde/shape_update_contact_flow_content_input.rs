// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_contact_flow_content_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateContactFlowContentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.content {
        object.key("Content").string(var_1.as_str());
    }
    Ok(())
}

