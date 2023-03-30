// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_device_fleets_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListDeviceFleetsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.next_token {
        object.key("NextToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.creation_time_after {
        object.key("CreationTimeAfter").date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.creation_time_before {
        object.key("CreationTimeBefore").date_time(var_4, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_5) = &input.last_modified_time_after {
        object.key("LastModifiedTimeAfter").date_time(var_5, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_6) = &input.last_modified_time_before {
        object.key("LastModifiedTimeBefore").date_time(var_6, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_7) = &input.name_contains {
        object.key("NameContains").string(var_7.as_str());
    }
    if let Some(var_8) = &input.sort_by {
        object.key("SortBy").string(var_8.as_str());
    }
    if let Some(var_9) = &input.sort_order {
        object.key("SortOrder").string(var_9.as_str());
    }
    Ok(())
}

