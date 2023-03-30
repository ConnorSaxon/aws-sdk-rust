// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_generate_embed_url_for_registered_user_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GenerateEmbedUrlForRegisteredUserInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.allowed_domains {
        let mut array_2 = object.key("AllowedDomains").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.experience_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("ExperienceConfiguration").start_object();
        crate::protocol_serde::shape_registered_user_embedding_experience_configuration::ser_registered_user_embedding_experience_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.session_lifetime_in_minutes {
        object.key("SessionLifetimeInMinutes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    if let Some(var_7) = &input.user_arn {
        object.key("UserArn").string(var_7.as_str());
    }
    Ok(())
}

