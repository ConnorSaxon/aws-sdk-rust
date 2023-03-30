// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_dedicated_ip_pool_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateDedicatedIpPoolInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pool_name {
        object.key("PoolName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tags {
        let mut array_3 = object.key("Tags").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

