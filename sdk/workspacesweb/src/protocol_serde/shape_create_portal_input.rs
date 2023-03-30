// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_portal_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreatePortalInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.additional_encryption_context {
        #[allow(unused_mut)]
        let mut object_2 = object.key("additionalEncryptionContext").start_object();
        for (key_3, value_4) in var_1 {
             {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.authentication_type {
        object.key("authenticationType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.client_token {
        object.key("clientToken").string(var_6.as_str());
    }
    if let Some(var_7) = &input.customer_managed_key {
        object.key("customerManagedKey").string(var_7.as_str());
    }
    if let Some(var_8) = &input.display_name {
        object.key("displayName").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        let mut array_10 = object.key("tags").start_array();
        for item_11 in var_9 {
             {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    Ok(())
}

