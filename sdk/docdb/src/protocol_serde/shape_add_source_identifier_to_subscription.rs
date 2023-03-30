// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_add_source_identifier_to_subscription_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AddSourceIdentifierToSubscriptionOutput, crate::error::AddSourceIdentifierToSubscriptionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AddSourceIdentifierToSubscriptionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::AddSourceIdentifierToSubscriptionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "SourceNotFound" => crate::error::AddSourceIdentifierToSubscriptionError::SourceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::source_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_source_not_found_fault::de_source_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::AddSourceIdentifierToSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SubscriptionNotFound" => crate::error::AddSourceIdentifierToSubscriptionError::SubscriptionNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::subscription_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_subscription_not_found_fault::de_subscription_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::AddSourceIdentifierToSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::AddSourceIdentifierToSubscriptionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_add_source_identifier_to_subscription_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AddSourceIdentifierToSubscriptionOutput, crate::error::AddSourceIdentifierToSubscriptionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::add_source_identifier_to_subscription_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_add_source_identifier_to_subscription::de_add_source_identifier_to_subscription(response.body().as_ref(), output).map_err(crate::error::AddSourceIdentifierToSubscriptionError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_add_source_identifier_to_subscription(inp: &[u8], mut builder: crate::output::add_source_identifier_to_subscription_output::Builder) -> Result<crate::output::add_source_identifier_to_subscription_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("AddSourceIdentifierToSubscriptionResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected AddSourceIdentifierToSubscriptionResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("AddSourceIdentifierToSubscriptionResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected AddSourceIdentifierToSubscriptionResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("EventSubscription") /* EventSubscription com.amazonaws.docdb.synthetic#AddSourceIdentifierToSubscriptionOutput$EventSubscription */ =>  {
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
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected AddSourceIdentifierToSubscriptionResult tag"))
                    };
    Ok(builder)
}

