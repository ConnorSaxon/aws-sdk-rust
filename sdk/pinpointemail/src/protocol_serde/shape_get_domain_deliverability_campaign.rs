// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_domain_deliverability_campaign_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetDomainDeliverabilityCampaignOutput, crate::error::GetDomainDeliverabilityCampaignError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetDomainDeliverabilityCampaignError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetDomainDeliverabilityCampaignError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::GetDomainDeliverabilityCampaignError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDomainDeliverabilityCampaignError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::error::GetDomainDeliverabilityCampaignError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDomainDeliverabilityCampaignError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::GetDomainDeliverabilityCampaignError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDomainDeliverabilityCampaignError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetDomainDeliverabilityCampaignError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_domain_deliverability_campaign_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetDomainDeliverabilityCampaignOutput, crate::error::GetDomainDeliverabilityCampaignError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_domain_deliverability_campaign_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_domain_deliverability_campaign::de_get_domain_deliverability_campaign(response.body().as_ref(), output).map_err(crate::error::GetDomainDeliverabilityCampaignError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_domain_deliverability_campaign(value: &[u8], mut builder: crate::output::get_domain_deliverability_campaign_output::Builder) -> Result<crate::output::get_domain_deliverability_campaign_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "DomainDeliverabilityCampaign" => {
                        builder = builder.set_domain_deliverability_campaign(
                            crate::protocol_serde::shape_domain_deliverability_campaign::de_domain_deliverability_campaign(tokens)?
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

