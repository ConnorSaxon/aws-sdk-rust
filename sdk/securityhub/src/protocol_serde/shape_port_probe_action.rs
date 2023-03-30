// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_port_probe_action(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PortProbeAction) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.port_probe_details {
        let mut array_2 = object.key("PortProbeDetails").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_port_probe_detail::ser_port_probe_detail(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if input.blocked {
        object.key("Blocked").boolean(input.blocked);
    }
    Ok(())
}

pub(crate) fn de_port_probe_action<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PortProbeAction>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::port_probe_action::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "PortProbeDetails" => {
                                builder = builder.set_port_probe_details(
                                    crate::protocol_serde::shape_port_probe_detail_list::de_port_probe_detail_list(tokens)?
                                );
                            }
                            "Blocked" => {
                                builder = builder.set_blocked(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

