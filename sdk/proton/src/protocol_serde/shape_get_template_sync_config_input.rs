// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_template_sync_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetTemplateSyncConfigInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.template_name {
        object.key("templateName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.template_type {
        object.key("templateType").string(var_2.as_str());
    }
    Ok(())
}

