// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_relational_database_metric_data_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetRelationalDatabaseMetricDataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.relational_database_name {
        object.key("relationalDatabaseName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.metric_name {
        object.key("metricName").string(var_2.as_str());
    }
     {
        object.key("period").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.period).into()));
    }
    if let Some(var_3) = &input.start_time {
        object.key("startTime").date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.end_time {
        object.key("endTime").date_time(var_4, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_5) = &input.unit {
        object.key("unit").string(var_5.as_str());
    }
    if let Some(var_6) = &input.statistics {
        let mut array_7 = object.key("statistics").start_array();
        for item_8 in var_6 {
             {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    Ok(())
}

