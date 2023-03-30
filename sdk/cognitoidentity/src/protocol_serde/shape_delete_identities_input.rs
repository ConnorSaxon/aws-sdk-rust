// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_identities_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteIdentitiesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_ids_to_delete {
        let mut array_2 = object.key("IdentityIdsToDelete").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}

