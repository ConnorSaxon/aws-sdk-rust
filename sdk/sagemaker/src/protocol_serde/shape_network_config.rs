// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_network_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NetworkConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.enable_inter_container_traffic_encryption {
        object.key("EnableInterContainerTrafficEncryption").boolean(input.enable_inter_container_traffic_encryption);
    }
    if input.enable_network_isolation {
        object.key("EnableNetworkIsolation").boolean(input.enable_network_isolation);
    }
    if let Some(var_1) = &input.vpc_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("VpcConfig").start_object();
        crate::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_network_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::NetworkConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::network_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "EnableInterContainerTrafficEncryption" => {
                                builder = builder.set_enable_inter_container_traffic_encryption(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "EnableNetworkIsolation" => {
                                builder = builder.set_enable_network_isolation(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "VpcConfig" => {
                                builder = builder.set_vpc_config(
                                    crate::protocol_serde::shape_vpc_config::de_vpc_config(tokens)?
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

