// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_direct_connect_gateway_associations_input(input: &crate::input::DescribeDirectConnectGatewayAssociationsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_direct_connect_gateway_associations_input::ser_describe_direct_connect_gateway_associations_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_direct_connect_gateway_associations_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeDirectConnectGatewayAssociationsOutput, crate::error::DescribeDirectConnectGatewayAssociationsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeDirectConnectGatewayAssociationsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeDirectConnectGatewayAssociationsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DirectConnectClientException" => crate::error::DescribeDirectConnectGatewayAssociationsError::DirectConnectClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::direct_connect_client_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_direct_connect_client_exception::de_direct_connect_client_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeDirectConnectGatewayAssociationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DirectConnectServerException" => crate::error::DescribeDirectConnectGatewayAssociationsError::DirectConnectServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::direct_connect_server_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_direct_connect_server_exception::de_direct_connect_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeDirectConnectGatewayAssociationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeDirectConnectGatewayAssociationsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_direct_connect_gateway_associations_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeDirectConnectGatewayAssociationsOutput, crate::error::DescribeDirectConnectGatewayAssociationsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_direct_connect_gateway_associations_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_direct_connect_gateway_associations::de_describe_direct_connect_gateway_associations(response.body().as_ref(), output).map_err(crate::error::DescribeDirectConnectGatewayAssociationsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_direct_connect_gateway_associations(value: &[u8], mut builder: crate::output::describe_direct_connect_gateway_associations_output::Builder) -> Result<crate::output::describe_direct_connect_gateway_associations_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "directConnectGatewayAssociations" => {
                        builder = builder.set_direct_connect_gateway_associations(
                            crate::protocol_serde::shape_direct_connect_gateway_association_list::de_direct_connect_gateway_association_list(tokens)?
                        );
                    }
                    "nextToken" => {
                        builder = builder.set_next_token(
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
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

