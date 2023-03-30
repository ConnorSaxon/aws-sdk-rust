// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_target_group_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyTargetGroupOutput, crate::error::ModifyTargetGroupError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ModifyTargetGroupError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ModifyTargetGroupError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidConfigurationRequest" => crate::error::ModifyTargetGroupError::InvalidConfigurationRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_configuration_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_configuration_request_exception::de_invalid_configuration_request_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyTargetGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TargetGroupNotFound" => crate::error::ModifyTargetGroupError::TargetGroupNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::target_group_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_target_group_not_found_exception::de_target_group_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyTargetGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ModifyTargetGroupError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_target_group_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyTargetGroupOutput, crate::error::ModifyTargetGroupError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::modify_target_group_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_modify_target_group::de_modify_target_group(response.body().as_ref(), output).map_err(crate::error::ModifyTargetGroupError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_target_group(inp: &[u8], mut builder: crate::output::modify_target_group_output::Builder) -> Result<crate::output::modify_target_group_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("ModifyTargetGroupResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ModifyTargetGroupResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("ModifyTargetGroupResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected ModifyTargetGroupResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("TargetGroups") /* TargetGroups com.amazonaws.elasticloadbalancingv2.synthetic#ModifyTargetGroupOutput$TargetGroups */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_target_groups::de_target_groups(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_target_groups(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected ModifyTargetGroupResult tag"))
                    };
    Ok(builder)
}

