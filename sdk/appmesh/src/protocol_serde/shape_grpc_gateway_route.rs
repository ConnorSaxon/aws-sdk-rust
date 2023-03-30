// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_grpc_gateway_route(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::GrpcGatewayRoute) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.r#match {
        #[allow(unused_mut)]
        let mut object_2 = object.key("match").start_object();
        crate::protocol_serde::shape_grpc_gateway_route_match::ser_grpc_gateway_route_match(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.action {
        #[allow(unused_mut)]
        let mut object_4 = object.key("action").start_object();
        crate::protocol_serde::shape_grpc_gateway_route_action::ser_grpc_gateway_route_action(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_grpc_gateway_route<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::GrpcGatewayRoute>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::grpc_gateway_route::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "match" => {
                                builder = builder.set_match(
                                    crate::protocol_serde::shape_grpc_gateway_route_match::de_grpc_gateway_route_match(tokens)?
                                );
                            }
                            "action" => {
                                builder = builder.set_action(
                                    crate::protocol_serde::shape_grpc_gateway_route_action::de_grpc_gateway_route_action(tokens)?
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

