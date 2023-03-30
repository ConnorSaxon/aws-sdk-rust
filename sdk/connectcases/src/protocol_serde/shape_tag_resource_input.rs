// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.tags {
        #[allow(unused_mut)]
        let mut object_2 = object.key("tags").start_object();
        for (key_3, value_4) in var_1 {
            if let Some(var_5) = value_4 {
                object_2.key(key_3.as_str()).string(var_5.as_str());
            }
            else {
                object_2.key(key_3.as_str()).null();
            }
        }
        object_2.finish();
    }
    Ok(())
}

