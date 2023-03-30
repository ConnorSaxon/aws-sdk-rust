// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_select_object_content_headers(
                    input: &crate::input::SelectObjectContentInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.sse_customer_algorithm {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("sse_customer_algorithm", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-customer-algorithm", header_value);
                        }
    }
    if let Some(inner_3) = &input.sse_customer_key {
        let formatted_4 = inner_3.as_str();
                        if !formatted_4.is_empty() {
                            let header_value = formatted_4;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("sse_customer_key", format!(
                                "`{}` cannot be used as a header value: {}",
                                &"*** Sensitive Data Redacted ***",
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-customer-key", header_value);
                        }
    }
    if let Some(inner_5) = &input.sse_customer_key_md5 {
        let formatted_6 = inner_5.as_str();
                        if !formatted_6.is_empty() {
                            let header_value = formatted_6;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("sse_customer_key_md5", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-server-side-encryption-customer-key-MD5", header_value);
                        }
    }
    if let Some(inner_7) = &input.expected_bucket_owner {
        let formatted_8 = inner_7.as_str();
                        if !formatted_8.is_empty() {
                            let header_value = formatted_8;
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

pub fn ser_select_object_content_op_input(input: &crate::input::SelectObjectContentInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
     {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
                                #[allow(unused_mut)]
                                let mut root = writer.start_el("SelectObjectContentRequest").write_ns("http://s3.amazonaws.com/doc/2006-03-01/", None);
        crate::protocol_serde::shape_select_object_content_input::ser_select_object_content_input_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_select_object_content_http_response(op_response: &mut aws_smithy_http::operation::Response) -> std::result::Result<crate::output::SelectObjectContentOutput, crate::error::SelectObjectContentError> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::select_object_content_output::Builder::default();
        let _ = response;
        output = output.set_payload(
            Some(crate::protocol_serde::shape_select_object_content_output::de_payload_payload(response.body_mut())?)
        );
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(response).map(str::to_string));
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build().map_err(crate::error::SelectObjectContentError::unhandled)?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_select_object_content_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SelectObjectContentOutput, crate::error::SelectObjectContentError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::SelectObjectContentError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, response.headers());
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::SelectObjectContentError::generic(generic))
}

