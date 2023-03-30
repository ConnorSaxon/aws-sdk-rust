// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_get_asset_property_aggregates_input(input: &crate::input::BatchGetAssetPropertyAggregatesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_batch_get_asset_property_aggregates_input::ser_batch_get_asset_property_aggregates_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_asset_property_aggregates_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchGetAssetPropertyAggregatesOutput, crate::error::BatchGetAssetPropertyAggregatesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::BatchGetAssetPropertyAggregatesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::BatchGetAssetPropertyAggregatesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalFailureException" => crate::error::BatchGetAssetPropertyAggregatesError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetAssetPropertyAggregatesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::error::BatchGetAssetPropertyAggregatesError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetAssetPropertyAggregatesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::BatchGetAssetPropertyAggregatesError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetAssetPropertyAggregatesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::error::BatchGetAssetPropertyAggregatesError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetAssetPropertyAggregatesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::BatchGetAssetPropertyAggregatesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_asset_property_aggregates_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchGetAssetPropertyAggregatesOutput, crate::error::BatchGetAssetPropertyAggregatesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::batch_get_asset_property_aggregates_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_batch_get_asset_property_aggregates::de_batch_get_asset_property_aggregates(response.body().as_ref(), output).map_err(crate::error::BatchGetAssetPropertyAggregatesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_batch_get_asset_property_aggregates(value: &[u8], mut builder: crate::output::batch_get_asset_property_aggregates_output::Builder) -> Result<crate::output::batch_get_asset_property_aggregates_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "errorEntries" => {
                        builder = builder.set_error_entries(
                            crate::protocol_serde::shape_batch_get_asset_property_aggregates_error_entries::de_batch_get_asset_property_aggregates_error_entries(tokens)?
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
                    "skippedEntries" => {
                        builder = builder.set_skipped_entries(
                            crate::protocol_serde::shape_batch_get_asset_property_aggregates_skipped_entries::de_batch_get_asset_property_aggregates_skipped_entries(tokens)?
                        );
                    }
                    "successEntries" => {
                        builder = builder.set_success_entries(
                            crate::protocol_serde::shape_batch_get_asset_property_aggregates_success_entries::de_batch_get_asset_property_aggregates_success_entries(tokens)?
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

