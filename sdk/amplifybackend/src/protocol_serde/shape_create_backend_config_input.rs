// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_backend_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateBackendConfigInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.backend_manager_app_id {
        object.key("backendManagerAppId").string(var_1.as_str());
    }
    Ok(())
}

