// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_virtual_node_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateVirtualNodeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.spec {
        #[allow(unused_mut)]
        let mut object_3 = object.key("spec").start_object();
        crate::protocol_serde::shape_virtual_node_spec::ser_virtual_node_spec(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("tags").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_tag_ref::ser_tag_ref(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.virtual_node_name {
        object.key("virtualNodeName").string(var_8.as_str());
    }
    Ok(())
}

