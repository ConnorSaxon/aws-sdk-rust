// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_trails_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeTrailsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.trail_name_list {
        let mut array_2 = object.key("trailNameList").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.include_shadow_trails {
        object.key("includeShadowTrails").boolean(*var_4);
    }
    Ok(())
}

