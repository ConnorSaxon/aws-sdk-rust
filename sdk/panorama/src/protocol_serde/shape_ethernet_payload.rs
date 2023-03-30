// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_ethernet_payload<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::EthernetPayload>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::ethernet_payload::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ConnectionType" => {
                                builder = builder.set_connection_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ConnectionType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "StaticIpConnectionInfo" => {
                                builder = builder.set_static_ip_connection_info(
                                    crate::protocol_serde::shape_static_ip_connection_info::de_static_ip_connection_info(tokens)?
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

pub fn ser_ethernet_payload(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EthernetPayload) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.connection_type {
        object.key("ConnectionType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.static_ip_connection_info {
        #[allow(unused_mut)]
        let mut object_3 = object.key("StaticIpConnectionInfo").start_object();
        crate::protocol_serde::shape_static_ip_connection_info::ser_static_ip_connection_info(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

