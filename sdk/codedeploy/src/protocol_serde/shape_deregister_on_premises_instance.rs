// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deregister_on_premises_instance_input(input: &crate::input::DeregisterOnPremisesInstanceInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_deregister_on_premises_instance_input::ser_deregister_on_premises_instance_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_on_premises_instance_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeregisterOnPremisesInstanceOutput, crate::error::DeregisterOnPremisesInstanceError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeregisterOnPremisesInstanceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeregisterOnPremisesInstanceError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InstanceNameRequiredException" => crate::error::DeregisterOnPremisesInstanceError::InstanceNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::instance_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_instance_name_required_exception::de_instance_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeregisterOnPremisesInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInstanceNameException" => crate::error::DeregisterOnPremisesInstanceError::InvalidInstanceNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_instance_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_instance_name_exception::de_invalid_instance_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeregisterOnPremisesInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeregisterOnPremisesInstanceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_on_premises_instance_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeregisterOnPremisesInstanceOutput, crate::error::DeregisterOnPremisesInstanceError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::deregister_on_premises_instance_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

