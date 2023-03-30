// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_network_origin_configuration(object_5: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NetworkOriginConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::NetworkOriginConfiguration::VpcConfiguration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_1 = object_5.key("vpcConfiguration").start_object();
                crate::protocol_serde::shape_vpc_configuration::ser_vpc_configuration(&mut object_1, inner)?;
                object_1.finish();
            }
        },
        crate::model::NetworkOriginConfiguration::InternetConfiguration(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_2 = object_5.key("internetConfiguration").start_object();
                crate::protocol_serde::shape_internet_configuration::ser_internet_configuration(&mut object_2, inner)?;
                object_2.finish();
            }
        },
        crate::model::NetworkOriginConfiguration::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("NetworkOriginConfiguration"))
    }
    Ok(())
}

pub(crate) fn de_network_origin_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::NetworkOriginConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
                                Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        if variant.is_some() {
                                                            return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("encountered mixed variants in union"));
                                                        }
                        variant = match key.to_unescaped()?.as_ref() {
                            "vpcConfiguration" => {
                                Some(crate::model::NetworkOriginConfiguration::VpcConfiguration(
                                    crate::protocol_serde::shape_vpc_configuration::de_vpc_configuration(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'vpcConfiguration' cannot be null"))?
                                ))
                            }
                            "internetConfiguration" => {
                                Some(crate::model::NetworkOriginConfiguration::InternetConfiguration(
                                    crate::protocol_serde::shape_internet_configuration::de_internet_configuration(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'internetConfiguration' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                      aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::model::NetworkOriginConfiguration::Unknown)
                                                                    }
                        };
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
        }
        _ => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
    }
    Ok(variant)
}

