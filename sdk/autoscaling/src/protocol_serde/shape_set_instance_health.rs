// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_set_instance_health_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SetInstanceHealthOutput, crate::error::SetInstanceHealthError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::SetInstanceHealthError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::SetInstanceHealthError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ResourceContention" => crate::error::SetInstanceHealthError::ResourceContentionFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_contention_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_contention_fault::de_resource_contention_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::SetInstanceHealthError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::SetInstanceHealthError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_set_instance_health_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SetInstanceHealthOutput, crate::error::SetInstanceHealthError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::set_instance_health_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

