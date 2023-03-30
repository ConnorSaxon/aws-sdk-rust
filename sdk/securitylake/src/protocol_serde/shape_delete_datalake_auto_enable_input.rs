// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_datalake_auto_enable_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteDatalakeAutoEnableInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.remove_from_configuration_for_new_accounts {
        let mut array_2 = object.key("removeFromConfigurationForNewAccounts").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_auto_enable_new_region_configuration::ser_auto_enable_new_region_configuration(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

