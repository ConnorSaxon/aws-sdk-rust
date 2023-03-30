// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateProfileInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.profile_id {
        object.key("ProfileId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.certificate_ids {
        let mut array_3 = object.key("CertificateIds").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}

