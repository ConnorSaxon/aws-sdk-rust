// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_app_launch_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteAppLaunchConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.app_id {
        object.key("appId").string(var_1.as_str());
    }
    Ok(())
}

