// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_instance_patch_states_for_patch_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeInstancePatchStatesForPatchGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.patch_group {
        object.key("PatchGroup").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filters {
        let mut array_3 = object.key("Filters").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_instance_patch_state_filter::ser_instance_patch_state_filter(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.next_token {
        object.key("NextToken").string(var_6.as_str());
    }
    if let Some(var_7) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    Ok(())
}

