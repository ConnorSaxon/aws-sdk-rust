// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_vpc_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VpcConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.security_group_ids {
        let mut array_2 = object.key("securityGroupIds").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.subnet_ids {
        let mut array_5 = object.key("subnetIds").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.vpc_id {
        object.key("vpcId").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_vpc_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::VpcConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::vpc_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "securityGroupIds" => {
                                builder = builder.set_security_group_ids(
                                    crate::protocol_serde::shape_vpc_security_group_ids::de_vpc_security_group_ids(tokens)?
                                );
                            }
                            "subnetIds" => {
                                builder = builder.set_subnet_ids(
                                    crate::protocol_serde::shape_vpc_subnet_ids::de_vpc_subnet_ids(tokens)?
                                );
                            }
                            "vpcId" => {
                                builder = builder.set_vpc_id(
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

