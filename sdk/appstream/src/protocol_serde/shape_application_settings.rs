// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_application_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ApplicationSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("Enabled").boolean(input.enabled);
    }
    if let Some(var_1) = &input.settings_group {
        object.key("SettingsGroup").string(var_1.as_str());
    }
    Ok(())
}

