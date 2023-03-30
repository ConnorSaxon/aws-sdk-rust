// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_detach_instances_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DetachInstancesOutput, crate::error::DetachInstancesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DetachInstancesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DetachInstancesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ResourceContention" => crate::error::DetachInstancesError::ResourceContentionFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_contention_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_contention_fault::de_resource_contention_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DetachInstancesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DetachInstancesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_detach_instances_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DetachInstancesOutput, crate::error::DetachInstancesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::detach_instances_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_detach_instances::de_detach_instances(response.body().as_ref(), output).map_err(crate::error::DetachInstancesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_detach_instances(inp: &[u8], mut builder: crate::output::detach_instances_output::Builder) -> Result<crate::output::detach_instances_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DetachInstancesResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DetachInstancesResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DetachInstancesResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DetachInstancesResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Activities") /* Activities com.amazonaws.autoscaling.synthetic#DetachInstancesOutput$Activities */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_activities::de_activities(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_activities(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DetachInstancesResult tag"))
                    };
    Ok(builder)
}

