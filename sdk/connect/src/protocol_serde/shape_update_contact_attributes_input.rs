// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_contact_attributes_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateContactAttributesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Attributes").start_object();
        for (key_3, value_4) in var_1 {
             {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.initial_contact_id {
        object.key("InitialContactId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.instance_id {
        object.key("InstanceId").string(var_6.as_str());
    }
    Ok(())
}

