// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_dashboard_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutDashboardOutput, crate::error::PutDashboardError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PutDashboardError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::PutDashboardError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParameterInput" => crate::error::PutDashboardError::DashboardInvalidInputError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::dashboard_invalid_input_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_dashboard_invalid_input_error::de_dashboard_invalid_input_error_xml_err(response.body().as_ref(), output).map_err(crate::error::PutDashboardError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceError" => crate::error::PutDashboardError::InternalServiceFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_fault::de_internal_service_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::PutDashboardError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::PutDashboardError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_dashboard_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutDashboardOutput, crate::error::PutDashboardError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_dashboard_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_put_dashboard::de_put_dashboard(response.body().as_ref(), output).map_err(crate::error::PutDashboardError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_put_dashboard(inp: &[u8], mut builder: crate::output::put_dashboard_output::Builder) -> Result<crate::output::put_dashboard_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("PutDashboardResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected PutDashboardResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("PutDashboardResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected PutDashboardResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("DashboardValidationMessages") /* DashboardValidationMessages com.amazonaws.cloudwatch.synthetic#PutDashboardOutput$DashboardValidationMessages */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_dashboard_validation_messages::de_dashboard_validation_messages(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_dashboard_validation_messages(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected PutDashboardResult tag"))
                    };
    Ok(builder)
}

