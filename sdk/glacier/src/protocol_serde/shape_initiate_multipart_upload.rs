// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_initiate_multipart_upload_headers(
                    input: &crate::input::InitiateMultipartUploadInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.archive_description {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("archive_description", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-archive-description", header_value);
                        }
    }
    if let Some(inner_3) = &input.part_size {
        let formatted_4 = inner_3.as_str();
                        if !formatted_4.is_empty() {
                            let header_value = formatted_4;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("part_size", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-part-size", header_value);
                        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_initiate_multipart_upload_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::InitiateMultipartUploadOutput, crate::error::InitiateMultipartUploadError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::InitiateMultipartUploadError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::InitiateMultipartUploadError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParameterValueException" => crate::error::InitiateMultipartUploadError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InitiateMultipartUploadError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MissingParameterValueException" => crate::error::InitiateMultipartUploadError::MissingParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::missing_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_missing_parameter_value_exception::de_missing_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InitiateMultipartUploadError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::InitiateMultipartUploadError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InitiateMultipartUploadError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::InitiateMultipartUploadError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InitiateMultipartUploadError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::InitiateMultipartUploadError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_initiate_multipart_upload_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::InitiateMultipartUploadOutput, crate::error::InitiateMultipartUploadError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::initiate_multipart_upload_output::Builder::default();
        let _ = response;
        output = output.set_location(
            crate::protocol_serde::shape_initiate_multipart_upload_output::de_location_header(response.headers())
                                    .map_err(|_|crate::error::InitiateMultipartUploadError::unhandled("Failed to parse location from header `Location"))?
        );
        output = output.set_upload_id(
            crate::protocol_serde::shape_initiate_multipart_upload_output::de_upload_id_header(response.headers())
                                    .map_err(|_|crate::error::InitiateMultipartUploadError::unhandled("Failed to parse uploadId from header `x-amz-multipart-upload-id"))?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

