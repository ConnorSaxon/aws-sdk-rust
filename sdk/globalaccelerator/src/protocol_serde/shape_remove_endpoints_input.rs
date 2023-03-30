// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_remove_endpoints_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RemoveEndpointsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.endpoint_identifiers {
        let mut array_2 = object.key("EndpointIdentifiers").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_endpoint_identifier::ser_endpoint_identifier(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_5.as_str());
    }
    Ok(())
}

