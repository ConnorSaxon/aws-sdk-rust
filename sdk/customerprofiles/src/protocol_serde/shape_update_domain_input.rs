// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_domain_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateDomainInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.dead_letter_queue_url {
        object.key("DeadLetterQueueUrl").string(var_1.as_str());
    }
    if let Some(var_2) = &input.default_encryption_key {
        object.key("DefaultEncryptionKey").string(var_2.as_str());
    }
    if let Some(var_3) = &input.default_expiration_days {
        object.key("DefaultExpirationDays").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.matching {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Matching").start_object();
        crate::protocol_serde::shape_matching_request::ser_matching_request(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.tags {
        #[allow(unused_mut)]
        let mut object_7 = object.key("Tags").start_object();
        for (key_8, value_9) in var_6 {
             {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    Ok(())
}

