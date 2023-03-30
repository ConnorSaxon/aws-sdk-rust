// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_label_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLabelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.label_group_name {
        object.key("LabelGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.start_time {
        object.key("StartTime").date_time(var_2, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_3) = &input.end_time {
        object.key("EndTime").date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.rating {
        object.key("Rating").string(var_4.as_str());
    }
    if let Some(var_5) = &input.fault_code {
        object.key("FaultCode").string(var_5.as_str());
    }
    if let Some(var_6) = &input.notes {
        object.key("Notes").string(var_6.as_str());
    }
    if let Some(var_7) = &input.equipment {
        object.key("Equipment").string(var_7.as_str());
    }
    if let Some(var_8) = &input.client_token {
        object.key("ClientToken").string(var_8.as_str());
    }
    Ok(())
}

