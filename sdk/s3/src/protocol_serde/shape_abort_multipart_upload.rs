// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_abort_multipart_upload_headers(
                    input: &crate::input::AbortMultipartUploadInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.request_payer {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("request_payer", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-request-payer", header_value);
                        }
    }
    if let Some(inner_3) = &input.expected_bucket_owner {
        let formatted_4 = inner_3.as_str();
                        if !formatted_4.is_empty() {
                            let header_value = formatted_4;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("expected_bucket_owner", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-expected-bucket-owner", header_value);
                        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_abort_multipart_upload_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AbortMultipartUploadOutput, crate::error::AbortMultipartUploadError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AbortMultipartUploadError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, response.headers());
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::AbortMultipartUploadError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "NoSuchUpload" => crate::error::AbortMultipartUploadError::NoSuchUpload({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_upload::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_upload::de_no_such_upload_xml_err(response.body().as_ref(), output).map_err(crate::error::AbortMultipartUploadError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::AbortMultipartUploadError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_abort_multipart_upload_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AbortMultipartUploadOutput, crate::error::AbortMultipartUploadError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::abort_multipart_upload_output::Builder::default();
        let _ = response;
        output = output.set_request_charged(
            crate::protocol_serde::shape_abort_multipart_upload_output::de_request_charged_header(response.headers())
                                    .map_err(|_|crate::error::AbortMultipartUploadError::unhandled("Failed to parse RequestCharged from header `x-amz-request-charged"))?
        );
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(response).map(str::to_string));
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

