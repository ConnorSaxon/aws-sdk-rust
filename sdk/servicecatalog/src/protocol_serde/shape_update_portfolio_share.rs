// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_portfolio_share_input(input: &crate::input::UpdatePortfolioShareInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_portfolio_share_input::ser_update_portfolio_share_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_portfolio_share_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdatePortfolioShareOutput, crate::error::UpdatePortfolioShareError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdatePortfolioShareError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdatePortfolioShareError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParametersException" => crate::error::UpdatePortfolioShareError::InvalidParametersException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameters_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameters_exception::de_invalid_parameters_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePortfolioShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidStateException" => crate::error::UpdatePortfolioShareError::InvalidStateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_state_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_state_exception::de_invalid_state_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePortfolioShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationNotSupportedException" => crate::error::UpdatePortfolioShareError::OperationNotSupportedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_not_supported_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_not_supported_exception::de_operation_not_supported_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePortfolioShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::UpdatePortfolioShareError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePortfolioShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdatePortfolioShareError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_portfolio_share_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdatePortfolioShareOutput, crate::error::UpdatePortfolioShareError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_portfolio_share_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_portfolio_share::de_update_portfolio_share(response.body().as_ref(), output).map_err(crate::error::UpdatePortfolioShareError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_update_portfolio_share(value: &[u8], mut builder: crate::output::update_portfolio_share_output::Builder) -> Result<crate::output::update_portfolio_share_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "PortfolioShareToken" => {
                        builder = builder.set_portfolio_share_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Status" => {
                        builder = builder.set_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ShareStatus::from(u.as_ref())
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

