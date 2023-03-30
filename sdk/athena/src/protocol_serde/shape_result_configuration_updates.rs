// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_result_configuration_updates(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ResultConfigurationUpdates) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.output_location {
        object.key("OutputLocation").string(var_1.as_str());
    }
    if let Some(var_2) = &input.remove_output_location {
        object.key("RemoveOutputLocation").boolean(*var_2);
    }
    if let Some(var_3) = &input.encryption_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("EncryptionConfiguration").start_object();
        crate::protocol_serde::shape_encryption_configuration::ser_encryption_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.remove_encryption_configuration {
        object.key("RemoveEncryptionConfiguration").boolean(*var_5);
    }
    if let Some(var_6) = &input.expected_bucket_owner {
        object.key("ExpectedBucketOwner").string(var_6.as_str());
    }
    if let Some(var_7) = &input.remove_expected_bucket_owner {
        object.key("RemoveExpectedBucketOwner").boolean(*var_7);
    }
    if let Some(var_8) = &input.acl_configuration {
        #[allow(unused_mut)]
        let mut object_9 = object.key("AclConfiguration").start_object();
        crate::protocol_serde::shape_acl_configuration::ser_acl_configuration(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.remove_acl_configuration {
        object.key("RemoveAclConfiguration").boolean(*var_10);
    }
    Ok(())
}

