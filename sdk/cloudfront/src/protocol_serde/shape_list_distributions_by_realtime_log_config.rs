// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_distributions_by_realtime_log_config_op_input(input: &crate::input::ListDistributionsByRealtimeLogConfigInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
     {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
                                #[allow(unused_mut)]
                                let mut root = writer.start_el("ListDistributionsByRealtimeLogConfigRequest").write_ns("http://cloudfront.amazonaws.com/doc/2020-05-31/", None);
        crate::protocol_serde::shape_list_distributions_by_realtime_log_config_input::ser_list_distributions_by_realtime_log_config_input_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_distributions_by_realtime_log_config_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListDistributionsByRealtimeLogConfigOutput, crate::error::ListDistributionsByRealtimeLogConfigError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListDistributionsByRealtimeLogConfigError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListDistributionsByRealtimeLogConfigError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidArgument" => crate::error::ListDistributionsByRealtimeLogConfigError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(response.body().as_ref(), output).map_err(crate::error::ListDistributionsByRealtimeLogConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListDistributionsByRealtimeLogConfigError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_distributions_by_realtime_log_config_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListDistributionsByRealtimeLogConfigOutput, crate::error::ListDistributionsByRealtimeLogConfigError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_distributions_by_realtime_log_config_output::Builder::default();
        let _ = response;
        output = output.set_distribution_list(
            crate::protocol_serde::shape_list_distributions_by_realtime_log_config_output::de_distribution_list_payload(response.body().as_ref())?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

