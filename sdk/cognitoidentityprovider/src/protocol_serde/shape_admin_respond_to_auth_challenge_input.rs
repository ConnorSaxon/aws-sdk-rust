// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_admin_respond_to_auth_challenge_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AdminRespondToAuthChallengeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.user_pool_id {
        object.key("UserPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_id {
        object.key("ClientId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.challenge_name {
        object.key("ChallengeName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.challenge_responses {
        #[allow(unused_mut)]
        let mut object_5 = object.key("ChallengeResponses").start_object();
        for (key_6, value_7) in var_4 {
             {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.session {
        object.key("Session").string(var_8.as_str());
    }
    if let Some(var_9) = &input.analytics_metadata {
        #[allow(unused_mut)]
        let mut object_10 = object.key("AnalyticsMetadata").start_object();
        crate::protocol_serde::shape_analytics_metadata_type::ser_analytics_metadata_type(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.context_data {
        #[allow(unused_mut)]
        let mut object_12 = object.key("ContextData").start_object();
        crate::protocol_serde::shape_context_data_type::ser_context_data_type(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.client_metadata {
        #[allow(unused_mut)]
        let mut object_14 = object.key("ClientMetadata").start_object();
        for (key_15, value_16) in var_13 {
             {
                object_14.key(key_15.as_str()).string(value_16.as_str());
            }
        }
        object_14.finish();
    }
    Ok(())
}

