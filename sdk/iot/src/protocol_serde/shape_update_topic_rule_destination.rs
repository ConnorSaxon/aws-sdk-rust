// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_topic_rule_destination_input(input: &crate::input::UpdateTopicRuleDestinationInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_topic_rule_destination_input::ser_update_topic_rule_destination_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_topic_rule_destination_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateTopicRuleDestinationOutput, crate::error::UpdateTopicRuleDestinationError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateTopicRuleDestinationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateTopicRuleDestinationError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ConflictingResourceUpdateException" => crate::error::UpdateTopicRuleDestinationError::ConflictingResourceUpdateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::conflicting_resource_update_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflicting_resource_update_exception::de_conflicting_resource_update_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateTopicRuleDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalException" => crate::error::UpdateTopicRuleDestinationError::InternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateTopicRuleDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::error::UpdateTopicRuleDestinationError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateTopicRuleDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::UpdateTopicRuleDestinationError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateTopicRuleDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedException" => crate::error::UpdateTopicRuleDestinationError::UnauthorizedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateTopicRuleDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateTopicRuleDestinationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_topic_rule_destination_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateTopicRuleDestinationOutput, crate::error::UpdateTopicRuleDestinationError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_topic_rule_destination_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

