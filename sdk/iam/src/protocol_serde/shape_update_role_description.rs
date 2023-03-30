// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_role_description_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateRoleDescriptionOutput, crate::error::UpdateRoleDescriptionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateRoleDescriptionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateRoleDescriptionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "NoSuchEntity" => crate::error::UpdateRoleDescriptionError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_entity_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoleDescriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailure" => crate::error::UpdateRoleDescriptionError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoleDescriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnmodifiableEntity" => crate::error::UpdateRoleDescriptionError::UnmodifiableEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unmodifiable_entity_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unmodifiable_entity_exception::de_unmodifiable_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateRoleDescriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateRoleDescriptionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_role_description_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateRoleDescriptionOutput, crate::error::UpdateRoleDescriptionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_role_description_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_role_description::de_update_role_description(response.body().as_ref(), output).map_err(crate::error::UpdateRoleDescriptionError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_update_role_description(inp: &[u8], mut builder: crate::output::update_role_description_output::Builder) -> Result<crate::output::update_role_description_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("UpdateRoleDescriptionResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected UpdateRoleDescriptionResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("UpdateRoleDescriptionResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected UpdateRoleDescriptionResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Role") /* Role com.amazonaws.iam.synthetic#UpdateRoleDescriptionOutput$Role */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_role::de_role(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_role(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected UpdateRoleDescriptionResult tag"))
                    };
    Ok(builder)
}

