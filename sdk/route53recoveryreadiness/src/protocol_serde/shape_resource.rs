// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resource(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Resource) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.component_id {
        object.key("componentId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.dns_target_resource {
        #[allow(unused_mut)]
        let mut object_3 = object.key("dnsTargetResource").start_object();
        crate::protocol_serde::shape_dns_target_resource::ser_dns_target_resource(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.readiness_scopes {
        let mut array_5 = object.key("readinessScopes").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.resource_arn {
        object.key("resourceArn").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_resource<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Resource>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::resource::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "componentId" => {
                                builder = builder.set_component_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "dnsTargetResource" => {
                                builder = builder.set_dns_target_resource(
                                    crate::protocol_serde::shape_dns_target_resource::de_dns_target_resource(tokens)?
                                );
                            }
                            "readinessScopes" => {
                                builder = builder.set_readiness_scopes(
                                    crate::protocol_serde::shape___list_of__string::de___list_of__string(tokens)?
                                );
                            }
                            "resourceArn" => {
                                builder = builder.set_resource_arn(
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

