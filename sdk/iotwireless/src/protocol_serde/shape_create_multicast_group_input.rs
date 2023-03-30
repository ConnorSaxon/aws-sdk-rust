// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_multicast_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateMulticastGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.lo_ra_wan {
        #[allow(unused_mut)]
        let mut object_4 = object.key("LoRaWAN").start_object();
        crate::protocol_serde::shape_lo_ra_wan_multicast::ser_lo_ra_wan_multicast(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.name {
        object.key("Name").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}

