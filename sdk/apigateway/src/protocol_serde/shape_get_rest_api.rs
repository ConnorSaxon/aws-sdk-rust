// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_rest_api_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetRestApiOutput, crate::error::GetRestApiError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetRestApiError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetRestApiError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::GetRestApiError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetRestApiError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::error::GetRestApiError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetRestApiError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::GetRestApiError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetRestApiError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::error::GetRestApiError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedException" => crate::error::GetRestApiError::UnauthorizedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetRestApiError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetRestApiError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_rest_api_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetRestApiOutput, crate::error::GetRestApiError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_rest_api_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_rest_api::de_get_rest_api(response.body().as_ref(), output).map_err(crate::error::GetRestApiError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_rest_api(value: &[u8], mut builder: crate::output::get_rest_api_output::Builder) -> Result<crate::output::get_rest_api_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "apiKeySource" => {
                        builder = builder.set_api_key_source(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ApiKeySourceType::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "binaryMediaTypes" => {
                        builder = builder.set_binary_media_types(
                            crate::protocol_serde::shape_list_of_string::de_list_of_string(tokens)?
                        );
                    }
                    "createdDate" => {
                        builder = builder.set_created_date(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                        );
                    }
                    "description" => {
                        builder = builder.set_description(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "disableExecuteApiEndpoint" => {
                        builder = builder.set_disable_execute_api_endpoint(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                        );
                    }
                    "endpointConfiguration" => {
                        builder = builder.set_endpoint_configuration(
                            crate::protocol_serde::shape_endpoint_configuration::de_endpoint_configuration(tokens)?
                        );
                    }
                    "id" => {
                        builder = builder.set_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "minimumCompressionSize" => {
                        builder = builder.set_minimum_compression_size(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "name" => {
                        builder = builder.set_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "policy" => {
                        builder = builder.set_policy(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "tags" => {
                        builder = builder.set_tags(
                            crate::protocol_serde::shape_map_of_string_to_string::de_map_of_string_to_string(tokens)?
                        );
                    }
                    "version" => {
                        builder = builder.set_version(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "warnings" => {
                        builder = builder.set_warnings(
                            crate::protocol_serde::shape_list_of_string::de_list_of_string(tokens)?
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

