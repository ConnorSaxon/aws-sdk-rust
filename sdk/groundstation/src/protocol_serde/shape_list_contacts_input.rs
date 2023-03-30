// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_contacts_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListContactsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.end_time {
        object.key("endTime").date_time(var_1, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_2) = &input.ground_station {
        object.key("groundStation").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.mission_profile_arn {
        object.key("missionProfileArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.next_token {
        object.key("nextToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.satellite_arn {
        object.key("satelliteArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.start_time {
        object.key("startTime").date_time(var_7, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_8) = &input.status_list {
        let mut array_9 = object.key("statusList").start_array();
        for item_10 in var_8 {
             {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    Ok(())
}

