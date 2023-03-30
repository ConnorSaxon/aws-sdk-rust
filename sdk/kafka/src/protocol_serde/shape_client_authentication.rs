// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_client_authentication(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ClientAuthentication) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.sasl {
        #[allow(unused_mut)]
        let mut object_2 = object.key("sasl").start_object();
        crate::protocol_serde::shape_sasl::ser_sasl(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.tls {
        #[allow(unused_mut)]
        let mut object_4 = object.key("tls").start_object();
        crate::protocol_serde::shape_tls::ser_tls(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.unauthenticated {
        #[allow(unused_mut)]
        let mut object_6 = object.key("unauthenticated").start_object();
        crate::protocol_serde::shape_unauthenticated::ser_unauthenticated(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_client_authentication<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ClientAuthentication>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::client_authentication::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "sasl" => {
                                builder = builder.set_sasl(
                                    crate::protocol_serde::shape_sasl::de_sasl(tokens)?
                                );
                            }
                            "tls" => {
                                builder = builder.set_tls(
                                    crate::protocol_serde::shape_tls::de_tls(tokens)?
                                );
                            }
                            "unauthenticated" => {
                                builder = builder.set_unauthenticated(
                                    crate::protocol_serde::shape_unauthenticated::de_unauthenticated(tokens)?
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

