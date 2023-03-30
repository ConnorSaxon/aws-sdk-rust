// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_robot_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateRobotInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.architecture {
        object.key("architecture").string(var_1.as_str());
    }
    if let Some(var_2) = &input.greengrass_group_id {
        object.key("greengrassGroupId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        #[allow(unused_mut)]
        let mut object_5 = object.key("tags").start_object();
        for (key_6, value_7) in var_4 {
             {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    Ok(())
}

