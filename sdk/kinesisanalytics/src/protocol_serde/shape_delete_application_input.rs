// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_application_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteApplicationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.application_name {
        object.key("ApplicationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.create_timestamp {
        object.key("CreateTimestamp").date_time(var_2, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

