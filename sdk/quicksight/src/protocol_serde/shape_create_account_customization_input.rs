// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_account_customization_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAccountCustomizationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_customization {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AccountCustomization").start_object();
        crate::protocol_serde::shape_account_customization::ser_account_customization(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.tags {
        let mut array_4 = object.key("Tags").start_array();
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

