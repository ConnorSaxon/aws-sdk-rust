// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_object_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteObjectOutput, crate::error::DeleteObjectError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteObjectError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteObjectError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ContainerNotFoundException" => crate::error::DeleteObjectError::ContainerNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::container_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_container_not_found_exception::de_container_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteObjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::DeleteObjectError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteObjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ObjectNotFoundException" => crate::error::DeleteObjectError::ObjectNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::object_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_object_not_found_exception::de_object_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteObjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteObjectError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_object_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteObjectOutput, crate::error::DeleteObjectError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_object_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

