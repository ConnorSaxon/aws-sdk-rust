// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_fleet_advisor_collector_input(input: &crate::input::DeleteFleetAdvisorCollectorInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_fleet_advisor_collector_input::ser_delete_fleet_advisor_collector_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_fleet_advisor_collector_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteFleetAdvisorCollectorOutput, crate::error::DeleteFleetAdvisorCollectorError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteFleetAdvisorCollectorError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteFleetAdvisorCollectorError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CollectorNotFoundFault" => crate::error::DeleteFleetAdvisorCollectorError::CollectorNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::collector_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_collector_not_found_fault::de_collector_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteFleetAdvisorCollectorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidResourceStateFault" => crate::error::DeleteFleetAdvisorCollectorError::InvalidResourceStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_resource_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_resource_state_fault::de_invalid_resource_state_fault_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteFleetAdvisorCollectorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteFleetAdvisorCollectorError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_fleet_advisor_collector_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteFleetAdvisorCollectorOutput, crate::error::DeleteFleetAdvisorCollectorError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_fleet_advisor_collector_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

