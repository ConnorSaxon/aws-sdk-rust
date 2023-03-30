// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_input_device_thumbnail_headers(
                    input: &crate::input::DescribeInputDeviceThumbnailInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.accept {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("accept", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("accept", header_value);
                        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_input_device_thumbnail_http_response(op_response: &mut aws_smithy_http::operation::Response) -> std::result::Result<crate::output::DescribeInputDeviceThumbnailOutput, crate::error::DescribeInputDeviceThumbnailError> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_input_device_thumbnail_output::Builder::default();
        let _ = response;
        output = output.set_body(
            Some(crate::protocol_serde::shape_describe_input_device_thumbnail_output::de_body_payload(response.body_mut())?)
        );
        output = output.set_content_length(
            crate::protocol_serde::shape_describe_input_device_thumbnail_output::de_content_length_header(response.headers())
                                    .map_err(|_|crate::error::DescribeInputDeviceThumbnailError::unhandled("Failed to parse ContentLength from header `Content-Length"))?
        );
        output = output.set_content_type(
            crate::protocol_serde::shape_describe_input_device_thumbnail_output::de_content_type_header(response.headers())
                                    .map_err(|_|crate::error::DescribeInputDeviceThumbnailError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output = output.set_e_tag(
            crate::protocol_serde::shape_describe_input_device_thumbnail_output::de_e_tag_header(response.headers())
                                    .map_err(|_|crate::error::DescribeInputDeviceThumbnailError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output = output.set_last_modified(
            crate::protocol_serde::shape_describe_input_device_thumbnail_output::de_last_modified_header(response.headers())
                                    .map_err(|_|crate::error::DescribeInputDeviceThumbnailError::unhandled("Failed to parse LastModified from header `Last-Modified"))?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_input_device_thumbnail_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeInputDeviceThumbnailOutput, crate::error::DescribeInputDeviceThumbnailError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeInputDeviceThumbnailError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeInputDeviceThumbnailError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadGatewayException" => crate::error::DescribeInputDeviceThumbnailError::BadGatewayException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_gateway_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_gateway_exception::de_bad_gateway_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeInputDeviceThumbnailError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "BadRequestException" => crate::error::DescribeInputDeviceThumbnailError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeInputDeviceThumbnailError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ForbiddenException" => crate::error::DescribeInputDeviceThumbnailError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::forbidden_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeInputDeviceThumbnailError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "GatewayTimeoutException" => crate::error::DescribeInputDeviceThumbnailError::GatewayTimeoutException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::gateway_timeout_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_gateway_timeout_exception::de_gateway_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeInputDeviceThumbnailError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerErrorException" => crate::error::DescribeInputDeviceThumbnailError::InternalServerErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeInputDeviceThumbnailError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::error::DescribeInputDeviceThumbnailError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeInputDeviceThumbnailError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::DescribeInputDeviceThumbnailError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeInputDeviceThumbnailError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeInputDeviceThumbnailError::generic(generic)
    })
}

