// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_worker_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateWorkerConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.properties_file_content {
        object.key("propertiesFileContent").string(var_3.as_str());
    }
    Ok(())
}

