// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_bucket_ownership_controls_headers(
                    input: &crate::input::PutBucketOwnershipControlsInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.content_md5 {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("content_md5", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("Content-MD5", header_value);
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
pub fn de_put_bucket_ownership_controls_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutBucketOwnershipControlsOutput, crate::error::PutBucketOwnershipControlsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PutBucketOwnershipControlsError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, response.headers());
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::PutBucketOwnershipControlsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_bucket_ownership_controls_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutBucketOwnershipControlsOutput, crate::error::PutBucketOwnershipControlsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_bucket_ownership_controls_output::Builder::default();
        let _ = response;
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(response).map(str::to_string));
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

