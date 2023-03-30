// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_service_connect_service(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ServiceConnectService) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.port_name {
        object.key("portName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.discovery_name {
        object.key("discoveryName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.client_aliases {
        let mut array_4 = object.key("clientAliases").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_service_connect_client_alias::ser_service_connect_client_alias(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.ingress_port_override {
        object.key("ingressPortOverride").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    Ok(())
}

pub(crate) fn de_service_connect_service<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ServiceConnectService>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::service_connect_service::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "portName" => {
                                builder = builder.set_port_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "discoveryName" => {
                                builder = builder.set_discovery_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "clientAliases" => {
                                builder = builder.set_client_aliases(
                                    crate::protocol_serde::shape_service_connect_client_alias_list::de_service_connect_client_alias_list(tokens)?
                                );
                            }
                            "ingressPortOverride" => {
                                builder = builder.set_ingress_port_override(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
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

