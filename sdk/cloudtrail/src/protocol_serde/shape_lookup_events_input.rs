// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lookup_events_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::LookupEventsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.lookup_attributes {
        let mut array_2 = object.key("LookupAttributes").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_lookup_attribute::ser_lookup_attribute(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.start_time {
        object.key("StartTime").date_time(var_5, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_6) = &input.end_time {
        object.key("EndTime").date_time(var_6, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_7) = &input.event_category {
        object.key("EventCategory").string(var_7.as_str());
    }
    if let Some(var_8) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    if let Some(var_9) = &input.next_token {
        object.key("NextToken").string(var_9.as_str());
    }
    Ok(())
}

