// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_test_function_headers(
                    input: &crate::input::TestFunctionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.if_match {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("if_match", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("If-Match", header_value);
                        }
    }
    Ok(builder)
}

pub fn ser_test_function_op_input(input: &crate::input::TestFunctionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
     {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
                                #[allow(unused_mut)]
                                let mut root = writer.start_el("TestFunctionRequest").write_ns("http://cloudfront.amazonaws.com/doc/2020-05-31/", None);
        crate::protocol_serde::shape_test_function_input::ser_test_function_input_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_test_function_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::TestFunctionOutput, crate::error::TestFunctionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::TestFunctionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::TestFunctionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidArgument" => crate::error::TestFunctionError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(response.body().as_ref(), output).map_err(crate::error::TestFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidIfMatchVersion" => crate::error::TestFunctionError::InvalidIfMatchVersion({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_if_match_version::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_if_match_version::de_invalid_if_match_version_xml_err(response.body().as_ref(), output).map_err(crate::error::TestFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchFunctionExists" => crate::error::TestFunctionError::NoSuchFunctionExists({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_function_exists::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_function_exists::de_no_such_function_exists_xml_err(response.body().as_ref(), output).map_err(crate::error::TestFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TestFunctionFailed" => crate::error::TestFunctionError::TestFunctionFailed({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::test_function_failed::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_test_function_failed::de_test_function_failed_xml_err(response.body().as_ref(), output).map_err(crate::error::TestFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedOperation" => crate::error::TestFunctionError::UnsupportedOperation({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_operation::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_operation::de_unsupported_operation_xml_err(response.body().as_ref(), output).map_err(crate::error::TestFunctionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::TestFunctionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_test_function_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::TestFunctionOutput, crate::error::TestFunctionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::test_function_output::Builder::default();
        let _ = response;
        output = output.set_test_result(
            crate::protocol_serde::shape_test_function_output::de_test_result_payload(response.body().as_ref())?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

