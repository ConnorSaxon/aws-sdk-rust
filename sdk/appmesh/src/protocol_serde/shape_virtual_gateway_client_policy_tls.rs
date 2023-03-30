// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_virtual_gateway_client_policy_tls(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VirtualGatewayClientPolicyTls) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.enforce {
        object.key("enforce").boolean(*var_1);
    }
    if let Some(var_2) = &input.ports {
        let mut array_3 = object.key("ports").start_array();
        for item_4 in var_2 {
             {
                array_3.value().number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*item_4).into()));
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.certificate {
        #[allow(unused_mut)]
        let mut object_6 = object.key("certificate").start_object();
        crate::protocol_serde::shape_virtual_gateway_client_tls_certificate::ser_virtual_gateway_client_tls_certificate(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.validation {
        #[allow(unused_mut)]
        let mut object_8 = object.key("validation").start_object();
        crate::protocol_serde::shape_virtual_gateway_tls_validation_context::ser_virtual_gateway_tls_validation_context(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}

pub(crate) fn de_virtual_gateway_client_policy_tls<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::VirtualGatewayClientPolicyTls>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::virtual_gateway_client_policy_tls::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "enforce" => {
                                builder = builder.set_enforce(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "ports" => {
                                builder = builder.set_ports(
                                    crate::protocol_serde::shape_port_set::de_port_set(tokens)?
                                );
                            }
                            "certificate" => {
                                builder = builder.set_certificate(
                                    crate::protocol_serde::shape_virtual_gateway_client_tls_certificate::de_virtual_gateway_client_tls_certificate(tokens)?
                                );
                            }
                            "validation" => {
                                builder = builder.set_validation(
                                    crate::protocol_serde::shape_virtual_gateway_tls_validation_context::de_virtual_gateway_tls_validation_context(tokens)?
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

