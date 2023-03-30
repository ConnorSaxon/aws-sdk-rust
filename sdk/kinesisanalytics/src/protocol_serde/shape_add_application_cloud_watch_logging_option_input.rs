// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_application_cloud_watch_logging_option_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AddApplicationCloudWatchLoggingOptionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.application_name {
        object.key("ApplicationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.cloud_watch_logging_option {
        #[allow(unused_mut)]
        let mut object_4 = object.key("CloudWatchLoggingOption").start_object();
        crate::protocol_serde::shape_cloud_watch_logging_option::ser_cloud_watch_logging_option(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

