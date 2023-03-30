// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_resource_set_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutResourceSetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_set {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ResourceSet").start_object();
        crate::protocol_serde::shape_resource_set::ser_resource_set(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.tag_list {
        let mut array_4 = object.key("TagList").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}

