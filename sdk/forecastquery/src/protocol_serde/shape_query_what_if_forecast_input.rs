// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_query_what_if_forecast_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::QueryWhatIfForecastInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.what_if_forecast_arn {
        object.key("WhatIfForecastArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.start_date {
        object.key("StartDate").string(var_2.as_str());
    }
    if let Some(var_3) = &input.end_date {
        object.key("EndDate").string(var_3.as_str());
    }
    if let Some(var_4) = &input.filters {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Filters").start_object();
        for (key_6, value_7) in var_4 {
             {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.next_token {
        object.key("NextToken").string(var_8.as_str());
    }
    Ok(())
}

