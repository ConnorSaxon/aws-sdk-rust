// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_what_if_analysis_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateWhatIfAnalysisInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.what_if_analysis_name {
        object.key("WhatIfAnalysisName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.forecast_arn {
        object.key("ForecastArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.time_series_selector {
        #[allow(unused_mut)]
        let mut object_4 = object.key("TimeSeriesSelector").start_object();
        crate::protocol_serde::shape_time_series_selector::ser_time_series_selector(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}

