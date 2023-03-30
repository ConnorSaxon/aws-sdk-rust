// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_model_cards_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListModelCardsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.creation_time_after {
        object.key("CreationTimeAfter").date_time(var_1, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_2) = &input.creation_time_before {
        object.key("CreationTimeBefore").date_time(var_2, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_3) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.name_contains {
        object.key("NameContains").string(var_4.as_str());
    }
    if let Some(var_5) = &input.model_card_status {
        object.key("ModelCardStatus").string(var_5.as_str());
    }
    if let Some(var_6) = &input.next_token {
        object.key("NextToken").string(var_6.as_str());
    }
    if let Some(var_7) = &input.sort_by {
        object.key("SortBy").string(var_7.as_str());
    }
    if let Some(var_8) = &input.sort_order {
        object.key("SortOrder").string(var_8.as_str());
    }
    Ok(())
}

