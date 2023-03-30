// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_blue_green_deployment_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteBlueGreenDeploymentOutput, crate::error::DeleteBlueGreenDeploymentError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteBlueGreenDeploymentError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteBlueGreenDeploymentError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BlueGreenDeploymentNotFoundFault" => crate::error::DeleteBlueGreenDeploymentError::BlueGreenDeploymentNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::blue_green_deployment_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_blue_green_deployment_not_found_fault::de_blue_green_deployment_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteBlueGreenDeploymentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidBlueGreenDeploymentStateFault" => crate::error::DeleteBlueGreenDeploymentError::InvalidBlueGreenDeploymentStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_blue_green_deployment_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_blue_green_deployment_state_fault::de_invalid_blue_green_deployment_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteBlueGreenDeploymentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteBlueGreenDeploymentError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_blue_green_deployment_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteBlueGreenDeploymentOutput, crate::error::DeleteBlueGreenDeploymentError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_blue_green_deployment_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_blue_green_deployment::de_delete_blue_green_deployment(response.body().as_ref(), output).map_err(crate::error::DeleteBlueGreenDeploymentError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_blue_green_deployment(inp: &[u8], mut builder: crate::output::delete_blue_green_deployment_output::Builder) -> Result<crate::output::delete_blue_green_deployment_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DeleteBlueGreenDeploymentResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DeleteBlueGreenDeploymentResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DeleteBlueGreenDeploymentResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DeleteBlueGreenDeploymentResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("BlueGreenDeployment") /* BlueGreenDeployment com.amazonaws.rds.synthetic#DeleteBlueGreenDeploymentOutput$BlueGreenDeployment */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_blue_green_deployment::de_blue_green_deployment(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_blue_green_deployment(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DeleteBlueGreenDeploymentResult tag"))
                    };
    Ok(builder)
}

