// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_web_acl_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateWebAclInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.scope {
        object.key("Scope").string(var_2.as_str());
    }
    if let Some(var_3) = &input.id {
        object.key("Id").string(var_3.as_str());
    }
    if let Some(var_4) = &input.default_action {
        #[allow(unused_mut)]
        let mut object_5 = object.key("DefaultAction").start_object();
        crate::protocol_serde::shape_default_action::ser_default_action(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.description {
        object.key("Description").string(var_6.as_str());
    }
    if let Some(var_7) = &input.rules {
        let mut array_8 = object.key("Rules").start_array();
        for item_9 in var_7 {
             {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_rule::ser_rule(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.visibility_config {
        #[allow(unused_mut)]
        let mut object_12 = object.key("VisibilityConfig").start_object();
        crate::protocol_serde::shape_visibility_config::ser_visibility_config(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.lock_token {
        object.key("LockToken").string(var_13.as_str());
    }
    if let Some(var_14) = &input.custom_response_bodies {
        #[allow(unused_mut)]
        let mut object_15 = object.key("CustomResponseBodies").start_object();
        for (key_16, value_17) in var_14 {
             {
                #[allow(unused_mut)]
                let mut object_18 = object_15.key(key_16.as_str()).start_object();
                crate::protocol_serde::shape_custom_response_body::ser_custom_response_body(&mut object_18, value_17)?;
                object_18.finish();
            }
        }
        object_15.finish();
    }
    if let Some(var_19) = &input.captcha_config {
        #[allow(unused_mut)]
        let mut object_20 = object.key("CaptchaConfig").start_object();
        crate::protocol_serde::shape_captcha_config::ser_captcha_config(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.challenge_config {
        #[allow(unused_mut)]
        let mut object_22 = object.key("ChallengeConfig").start_object();
        crate::protocol_serde::shape_challenge_config::ser_challenge_config(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.token_domains {
        let mut array_24 = object.key("TokenDomains").start_array();
        for item_25 in var_23 {
             {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    Ok(())
}

