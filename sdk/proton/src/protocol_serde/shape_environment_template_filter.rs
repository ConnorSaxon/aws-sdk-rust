// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_environment_template_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EnvironmentTemplateFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.template_name {
        object.key("templateName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.major_version {
        object.key("majorVersion").string(var_2.as_str());
    }
    Ok(())
}

