// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_initiate_auth_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::InitiateAuthInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.auth_flow {
        object.key("AuthFlow").string(var_1.as_str());
    }
    if let Some(var_2) = &input.auth_parameters {
        #[allow(unused_mut)]
        let mut object_3 = object.key("AuthParameters").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.client_metadata {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ClientMetadata").start_object();
        for (key_8, value_9) in var_6 {
             {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    if let Some(var_10) = &input.client_id {
        object.key("ClientId").string(var_10.as_str());
    }
    if let Some(var_11) = &input.analytics_metadata {
        #[allow(unused_mut)]
        let mut object_12 = object.key("AnalyticsMetadata").start_object();
        crate::protocol_serde::shape_analytics_metadata_type::ser_analytics_metadata_type(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.user_context_data {
        #[allow(unused_mut)]
        let mut object_14 = object.key("UserContextData").start_object();
        crate::protocol_serde::shape_user_context_data_type::ser_user_context_data_type(&mut object_14, var_13)?;
        object_14.finish();
    }
    Ok(())
}

