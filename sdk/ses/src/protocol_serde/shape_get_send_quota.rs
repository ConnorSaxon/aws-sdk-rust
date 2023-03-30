// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_send_quota_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetSendQuotaOutput, crate::error::GetSendQuotaError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetSendQuotaError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::GetSendQuotaError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_send_quota_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetSendQuotaOutput, crate::error::GetSendQuotaError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_send_quota_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_send_quota::de_get_send_quota(response.body().as_ref(), output).map_err(crate::error::GetSendQuotaError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_send_quota(inp: &[u8], mut builder: crate::output::get_send_quota_output::Builder) -> Result<crate::output::get_send_quota_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("GetSendQuotaResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected GetSendQuotaResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("GetSendQuotaResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected GetSendQuotaResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Max24HourSend") /* Max24HourSend com.amazonaws.ses.synthetic#GetSendQuotaOutput$Max24HourSend */ =>  {
                let var_1 =
                    Some(
                         {
                            <f64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.ses#Max24HourSend`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max24_hour_send(var_1);
            }
            ,
            s if s.matches("MaxSendRate") /* MaxSendRate com.amazonaws.ses.synthetic#GetSendQuotaOutput$MaxSendRate */ =>  {
                let var_2 =
                    Some(
                         {
                            <f64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.ses#MaxSendRate`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_send_rate(var_2);
            }
            ,
            s if s.matches("SentLast24Hours") /* SentLast24Hours com.amazonaws.ses.synthetic#GetSendQuotaOutput$SentLast24Hours */ =>  {
                let var_3 =
                    Some(
                         {
                            <f64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.ses#SentLast24Hours`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_sent_last24_hours(var_3);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected GetSendQuotaResult tag"))
                    };
    Ok(builder)
}

