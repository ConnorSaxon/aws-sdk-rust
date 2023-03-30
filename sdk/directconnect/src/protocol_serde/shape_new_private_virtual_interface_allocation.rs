// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_new_private_virtual_interface_allocation(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NewPrivateVirtualInterfaceAllocation) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.virtual_interface_name {
        object.key("virtualInterfaceName").string(var_1.as_str());
    }
     {
        object.key("vlan").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.vlan).into()));
    }
     {
        object.key("asn").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.asn).into()));
    }
    if let Some(var_2) = &input.mtu {
        object.key("mtu").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.auth_key {
        object.key("authKey").string(var_3.as_str());
    }
    if let Some(var_4) = &input.amazon_address {
        object.key("amazonAddress").string(var_4.as_str());
    }
    if let Some(var_5) = &input.address_family {
        object.key("addressFamily").string(var_5.as_str());
    }
    if let Some(var_6) = &input.customer_address {
        object.key("customerAddress").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        let mut array_8 = object.key("tags").start_array();
        for item_9 in var_7 {
             {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    Ok(())
}

