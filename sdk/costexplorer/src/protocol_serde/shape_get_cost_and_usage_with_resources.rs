// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_cost_and_usage_with_resources_input(input: &crate::input::GetCostAndUsageWithResourcesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_cost_and_usage_with_resources_input::ser_get_cost_and_usage_with_resources_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_cost_and_usage_with_resources_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetCostAndUsageWithResourcesOutput, crate::error::GetCostAndUsageWithResourcesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetCostAndUsageWithResourcesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetCostAndUsageWithResourcesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BillExpirationException" => crate::error::GetCostAndUsageWithResourcesError::BillExpirationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bill_expiration_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bill_expiration_exception::de_bill_expiration_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCostAndUsageWithResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DataUnavailableException" => crate::error::GetCostAndUsageWithResourcesError::DataUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::data_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_data_unavailable_exception::de_data_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCostAndUsageWithResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidNextTokenException" => crate::error::GetCostAndUsageWithResourcesError::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCostAndUsageWithResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::GetCostAndUsageWithResourcesError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCostAndUsageWithResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RequestChangedException" => crate::error::GetCostAndUsageWithResourcesError::RequestChangedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::request_changed_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_request_changed_exception::de_request_changed_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCostAndUsageWithResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetCostAndUsageWithResourcesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_cost_and_usage_with_resources_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetCostAndUsageWithResourcesOutput, crate::error::GetCostAndUsageWithResourcesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_cost_and_usage_with_resources_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_cost_and_usage_with_resources::de_get_cost_and_usage_with_resources(response.body().as_ref(), output).map_err(crate::error::GetCostAndUsageWithResourcesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_cost_and_usage_with_resources(value: &[u8], mut builder: crate::output::get_cost_and_usage_with_resources_output::Builder) -> Result<crate::output::get_cost_and_usage_with_resources_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "NextPageToken" => {
                        builder = builder.set_next_page_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "GroupDefinitions" => {
                        builder = builder.set_group_definitions(
                            crate::protocol_serde::shape_group_definitions::de_group_definitions(tokens)?
                        );
                    }
                    "ResultsByTime" => {
                        builder = builder.set_results_by_time(
                            crate::protocol_serde::shape_results_by_time::de_results_by_time(tokens)?
                        );
                    }
                    "DimensionValueAttributes" => {
                        builder = builder.set_dimension_value_attributes(
                            crate::protocol_serde::shape_dimension_values_with_attributes_list::de_dimension_values_with_attributes_list(tokens)?
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

