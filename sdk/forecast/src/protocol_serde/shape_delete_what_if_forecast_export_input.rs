// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_what_if_forecast_export_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteWhatIfForecastExportInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.what_if_forecast_export_arn {
        object.key("WhatIfForecastExportArn").string(var_1.as_str());
    }
    Ok(())
}

