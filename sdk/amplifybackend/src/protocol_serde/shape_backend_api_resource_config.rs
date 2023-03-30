// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_backend_api_resource_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BackendApiResourceConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.additional_auth_types {
        let mut array_2 = object.key("additionalAuthTypes").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_backend_api_auth_type::ser_backend_api_auth_type(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.api_name {
        object.key("apiName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.conflict_resolution {
        #[allow(unused_mut)]
        let mut object_7 = object.key("conflictResolution").start_object();
        crate::protocol_serde::shape_backend_api_conflict_resolution::ser_backend_api_conflict_resolution(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.default_auth_type {
        #[allow(unused_mut)]
        let mut object_9 = object.key("defaultAuthType").start_object();
        crate::protocol_serde::shape_backend_api_auth_type::ser_backend_api_auth_type(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.service {
        object.key("service").string(var_10.as_str());
    }
    if let Some(var_11) = &input.transform_schema {
        object.key("transformSchema").string(var_11.as_str());
    }
    Ok(())
}

pub(crate) fn de_backend_api_resource_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::BackendApiResourceConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::backend_api_resource_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "additionalAuthTypes" => {
                                builder = builder.set_additional_auth_types(
                                    crate::protocol_serde::shape_list_of_backend_api_auth_type::de_list_of_backend_api_auth_type(tokens)?
                                );
                            }
                            "apiName" => {
                                builder = builder.set_api_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "conflictResolution" => {
                                builder = builder.set_conflict_resolution(
                                    crate::protocol_serde::shape_backend_api_conflict_resolution::de_backend_api_conflict_resolution(tokens)?
                                );
                            }
                            "defaultAuthType" => {
                                builder = builder.set_default_auth_type(
                                    crate::protocol_serde::shape_backend_api_auth_type::de_backend_api_auth_type(tokens)?
                                );
                            }
                            "service" => {
                                builder = builder.set_service(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "transformSchema" => {
                                builder = builder.set_transform_schema(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
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

