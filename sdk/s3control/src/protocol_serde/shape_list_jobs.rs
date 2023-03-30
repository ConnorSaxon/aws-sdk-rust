// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_jobs_headers(
                    input: &crate::input::ListJobsInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.account_id {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("account_id", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-account-id", header_value);
                        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_jobs_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListJobsOutput, crate::error::ListJobsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListJobsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListJobsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::ListJobsError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::ListJobsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidNextTokenException" => crate::error::ListJobsError::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::ListJobsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::error::ListJobsError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::ListJobsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListJobsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_jobs_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListJobsOutput, crate::error::ListJobsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_jobs_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_jobs::de_list_jobs(response.body().as_ref(), output).map_err(crate::error::ListJobsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_jobs(inp: &[u8], mut builder: crate::output::list_jobs_output::Builder) -> Result<crate::output::list_jobs_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !start_el.matches("ListJobsResult") {
                            return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected ListJobsResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            )
                        }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("NextToken") /* NextToken com.amazonaws.s3control.synthetic#ListJobsOutput$NextToken */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_3);
            }
            ,
            s if s.matches("Jobs") /* Jobs com.amazonaws.s3control.synthetic#ListJobsOutput$Jobs */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_job_list_descriptor_list::de_job_list_descriptor_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_jobs(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

