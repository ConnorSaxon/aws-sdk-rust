// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_ledger_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLedgerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.deletion_protection {
        object.key("DeletionProtection").boolean(*var_1);
    }
    if let Some(var_2) = &input.kms_key {
        object.key("KmsKey").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.permissions_mode {
        object.key("PermissionsMode").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Tags").start_object();
        for (key_7, value_8) in var_5 {
            if let Some(var_9) = value_8 {
                object_6.key(key_7.as_str()).string(var_9.as_str());
            }
            else {
                object_6.key(key_7.as_str()).null();
            }
        }
        object_6.finish();
    }
    Ok(())
}

