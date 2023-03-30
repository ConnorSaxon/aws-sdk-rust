// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_user_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyUserOutput, crate::error::ModifyUserError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ModifyUserError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ModifyUserError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParameterCombination" => crate::error::ModifyUserError::InvalidParameterCombinationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_combination_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_combination_exception::de_invalid_parameter_combination_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValue" => crate::error::ModifyUserError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidUserState" => crate::error::ModifyUserError::InvalidUserStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_user_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_user_state_fault::de_invalid_user_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceLinkedRoleNotFoundFault" => crate::error::ModifyUserError::ServiceLinkedRoleNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_linked_role_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_linked_role_not_found_fault::de_service_linked_role_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UserNotFound" => crate::error::ModifyUserError::UserNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::user_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_not_found_fault::de_user_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ModifyUserError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_user_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyUserOutput, crate::error::ModifyUserError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::modify_user_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_modify_user::de_modify_user(response.body().as_ref(), output).map_err(crate::error::ModifyUserError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_user(inp: &[u8], mut builder: crate::output::modify_user_output::Builder) -> Result<crate::output::modify_user_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("ModifyUserResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ModifyUserResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("ModifyUserResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected ModifyUserResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("UserId") /* UserId com.amazonaws.elasticache.synthetic#ModifyUserOutput$UserId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_user_id(var_1);
            }
            ,
            s if s.matches("UserName") /* UserName com.amazonaws.elasticache.synthetic#ModifyUserOutput$UserName */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_user_name(var_2);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.elasticache.synthetic#ModifyUserOutput$Status */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_3);
            }
            ,
            s if s.matches("Engine") /* Engine com.amazonaws.elasticache.synthetic#ModifyUserOutput$Engine */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine(var_4);
            }
            ,
            s if s.matches("MinimumEngineVersion") /* MinimumEngineVersion com.amazonaws.elasticache.synthetic#ModifyUserOutput$MinimumEngineVersion */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_minimum_engine_version(var_5);
            }
            ,
            s if s.matches("AccessString") /* AccessString com.amazonaws.elasticache.synthetic#ModifyUserOutput$AccessString */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_access_string(var_6);
            }
            ,
            s if s.matches("UserGroupIds") /* UserGroupIds com.amazonaws.elasticache.synthetic#ModifyUserOutput$UserGroupIds */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_user_group_id_list::de_user_group_id_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_user_group_ids(var_7);
            }
            ,
            s if s.matches("Authentication") /* Authentication com.amazonaws.elasticache.synthetic#ModifyUserOutput$Authentication */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_authentication::de_authentication(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_authentication(var_8);
            }
            ,
            s if s.matches("ARN") /* ARN com.amazonaws.elasticache.synthetic#ModifyUserOutput$ARN */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_arn(var_9);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected ModifyUserResult tag"))
                    };
    Ok(builder)
}

