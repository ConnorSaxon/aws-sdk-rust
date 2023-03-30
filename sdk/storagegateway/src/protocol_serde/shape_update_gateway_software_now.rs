// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_gateway_software_now_input(input: &crate::input::UpdateGatewaySoftwareNowInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_gateway_software_now_input::ser_update_gateway_software_now_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_gateway_software_now_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateGatewaySoftwareNowOutput, crate::error::UpdateGatewaySoftwareNowError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateGatewaySoftwareNowError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateGatewaySoftwareNowError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerError" => crate::error::UpdateGatewaySoftwareNowError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateGatewaySoftwareNowError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidGatewayRequestException" => crate::error::UpdateGatewaySoftwareNowError::InvalidGatewayRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_gateway_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_gateway_request_exception::de_invalid_gateway_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateGatewaySoftwareNowError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateGatewaySoftwareNowError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_gateway_software_now_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateGatewaySoftwareNowOutput, crate::error::UpdateGatewaySoftwareNowError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_gateway_software_now_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_gateway_software_now::de_update_gateway_software_now(response.body().as_ref(), output).map_err(crate::error::UpdateGatewaySoftwareNowError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_update_gateway_software_now(value: &[u8], mut builder: crate::output::update_gateway_software_now_output::Builder) -> Result<crate::output::update_gateway_software_now_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "GatewayARN" => {
                        builder = builder.set_gateway_arn(
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

