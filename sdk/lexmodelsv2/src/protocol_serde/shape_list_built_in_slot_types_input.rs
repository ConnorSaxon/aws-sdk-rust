// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_built_in_slot_types_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListBuiltInSlotTypesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    if let Some(var_2) = &input.next_token {
        object.key("nextToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.sort_by {
        #[allow(unused_mut)]
        let mut object_4 = object.key("sortBy").start_object();
        crate::protocol_serde::shape_built_in_slot_type_sort_by::ser_built_in_slot_type_sort_by(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

