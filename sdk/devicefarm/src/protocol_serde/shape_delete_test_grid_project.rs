// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_test_grid_project_input(input: &crate::input::DeleteTestGridProjectInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_test_grid_project_input::ser_delete_test_grid_project_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_test_grid_project_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteTestGridProjectOutput, crate::error::DeleteTestGridProjectError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteTestGridProjectError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteTestGridProjectError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ArgumentException" => crate::error::DeleteTestGridProjectError::ArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::argument_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_argument_exception::de_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteTestGridProjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CannotDeleteException" => crate::error::DeleteTestGridProjectError::CannotDeleteException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cannot_delete_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cannot_delete_exception::de_cannot_delete_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteTestGridProjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceException" => crate::error::DeleteTestGridProjectError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteTestGridProjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::error::DeleteTestGridProjectError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteTestGridProjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteTestGridProjectError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_test_grid_project_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteTestGridProjectOutput, crate::error::DeleteTestGridProjectError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_test_grid_project_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

