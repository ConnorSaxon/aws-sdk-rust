// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_default_space_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DefaultSpaceSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.execution_role {
        object.key("ExecutionRole").string(var_1.as_str());
    }
    if let Some(var_2) = &input.security_groups {
        let mut array_3 = object.key("SecurityGroups").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.jupyter_server_app_settings {
        #[allow(unused_mut)]
        let mut object_6 = object.key("JupyterServerAppSettings").start_object();
        crate::protocol_serde::shape_jupyter_server_app_settings::ser_jupyter_server_app_settings(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.kernel_gateway_app_settings {
        #[allow(unused_mut)]
        let mut object_8 = object.key("KernelGatewayAppSettings").start_object();
        crate::protocol_serde::shape_kernel_gateway_app_settings::ser_kernel_gateway_app_settings(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}

pub(crate) fn de_default_space_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DefaultSpaceSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::default_space_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ExecutionRole" => {
                                builder = builder.set_execution_role(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SecurityGroups" => {
                                builder = builder.set_security_groups(
                                    crate::protocol_serde::shape_security_group_ids::de_security_group_ids(tokens)?
                                );
                            }
                            "JupyterServerAppSettings" => {
                                builder = builder.set_jupyter_server_app_settings(
                                    crate::protocol_serde::shape_jupyter_server_app_settings::de_jupyter_server_app_settings(tokens)?
                                );
                            }
                            "KernelGatewayAppSettings" => {
                                builder = builder.set_kernel_gateway_app_settings(
                                    crate::protocol_serde::shape_kernel_gateway_app_settings::de_kernel_gateway_app_settings(tokens)?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

