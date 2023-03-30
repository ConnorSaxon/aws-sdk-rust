// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_calendar_state_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetCalendarStateInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.calendar_names {
        let mut array_2 = object.key("CalendarNames").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.at_time {
        object.key("AtTime").string(var_4.as_str());
    }
    Ok(())
}

