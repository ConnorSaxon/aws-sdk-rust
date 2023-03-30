// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_event_subscription_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateEventSubscriptionOutput, crate::error::CreateEventSubscriptionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateEventSubscriptionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EventSubscriptionQuotaExceeded" => crate::error::CreateEventSubscriptionError::EventSubscriptionQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::event_subscription_quota_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_event_subscription_quota_exceeded_fault::de_event_subscription_quota_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTagFault" => crate::error::CreateEventSubscriptionError::InvalidTagFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_tag_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_tag_fault::de_invalid_tag_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SNSInvalidTopic" => crate::error::CreateEventSubscriptionError::SnsInvalidTopicFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::sns_invalid_topic_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_sns_invalid_topic_fault::de_sns_invalid_topic_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SNSNoAuthorization" => crate::error::CreateEventSubscriptionError::SnsNoAuthorizationFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::sns_no_authorization_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_sns_no_authorization_fault::de_sns_no_authorization_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SNSTopicArnNotFound" => crate::error::CreateEventSubscriptionError::SnsTopicArnNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::sns_topic_arn_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_sns_topic_arn_not_found_fault::de_sns_topic_arn_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SourceNotFound" => crate::error::CreateEventSubscriptionError::SourceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::source_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_source_not_found_fault::de_source_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SubscriptionAlreadyExist" => crate::error::CreateEventSubscriptionError::SubscriptionAlreadyExistFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::subscription_already_exist_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_subscription_already_exist_fault::de_subscription_already_exist_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SubscriptionCategoryNotFound" => crate::error::CreateEventSubscriptionError::SubscriptionCategoryNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::subscription_category_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_subscription_category_not_found_fault::de_subscription_category_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SubscriptionEventIdNotFound" => crate::error::CreateEventSubscriptionError::SubscriptionEventIdNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::subscription_event_id_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_subscription_event_id_not_found_fault::de_subscription_event_id_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SubscriptionSeverityNotFound" => crate::error::CreateEventSubscriptionError::SubscriptionSeverityNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::subscription_severity_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_subscription_severity_not_found_fault::de_subscription_severity_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TagLimitExceededFault" => crate::error::CreateEventSubscriptionError::TagLimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::tag_limit_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_tag_limit_exceeded_fault::de_tag_limit_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateEventSubscriptionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_event_subscription_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateEventSubscriptionOutput, crate::error::CreateEventSubscriptionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_event_subscription_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_event_subscription::de_create_event_subscription(response.body().as_ref(), output).map_err(crate::error::CreateEventSubscriptionError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_event_subscription(inp: &[u8], mut builder: crate::output::create_event_subscription_output::Builder) -> Result<crate::output::create_event_subscription_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("CreateEventSubscriptionResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected CreateEventSubscriptionResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("CreateEventSubscriptionResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected CreateEventSubscriptionResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("EventSubscription") /* EventSubscription com.amazonaws.redshift.synthetic#CreateEventSubscriptionOutput$EventSubscription */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_event_subscription::de_event_subscription(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_event_subscription(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected CreateEventSubscriptionResult tag"))
                    };
    Ok(builder)
}

