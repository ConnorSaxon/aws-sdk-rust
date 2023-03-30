// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_user_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteUserOutput, crate::error::DeleteUserError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteUserError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteUserError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DefaultUserAssociatedToUserGroup" => crate::error::DeleteUserError::DefaultUserAssociatedToUserGroupFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::default_user_associated_to_user_group_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_default_user_associated_to_user_group_fault::de_default_user_associated_to_user_group_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValue" => crate::error::DeleteUserError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidUserState" => crate::error::DeleteUserError::InvalidUserStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_user_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_user_state_fault::de_invalid_user_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceLinkedRoleNotFoundFault" => crate::error::DeleteUserError::ServiceLinkedRoleNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_linked_role_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_linked_role_not_found_fault::de_service_linked_role_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UserNotFound" => crate::error::DeleteUserError::UserNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::user_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_not_found_fault::de_user_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteUserError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_user_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteUserOutput, crate::error::DeleteUserError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_user_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_user::de_delete_user(response.body().as_ref(), output).map_err(crate::error::DeleteUserError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_user(inp: &[u8], mut builder: crate::output::delete_user_output::Builder) -> Result<crate::output::delete_user_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DeleteUserResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DeleteUserResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DeleteUserResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DeleteUserResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("UserId") /* UserId com.amazonaws.elasticache.synthetic#DeleteUserOutput$UserId */ =>  {
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
            s if s.matches("UserName") /* UserName com.amazonaws.elasticache.synthetic#DeleteUserOutput$UserName */ =>  {
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
            s if s.matches("Status") /* Status com.amazonaws.elasticache.synthetic#DeleteUserOutput$Status */ =>  {
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
            s if s.matches("Engine") /* Engine com.amazonaws.elasticache.synthetic#DeleteUserOutput$Engine */ =>  {
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
            s if s.matches("MinimumEngineVersion") /* MinimumEngineVersion com.amazonaws.elasticache.synthetic#DeleteUserOutput$MinimumEngineVersion */ =>  {
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
            s if s.matches("AccessString") /* AccessString com.amazonaws.elasticache.synthetic#DeleteUserOutput$AccessString */ =>  {
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
            s if s.matches("UserGroupIds") /* UserGroupIds com.amazonaws.elasticache.synthetic#DeleteUserOutput$UserGroupIds */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_user_group_id_list::de_user_group_id_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_user_group_ids(var_7);
            }
            ,
            s if s.matches("Authentication") /* Authentication com.amazonaws.elasticache.synthetic#DeleteUserOutput$Authentication */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_authentication::de_authentication(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_authentication(var_8);
            }
            ,
            s if s.matches("ARN") /* ARN com.amazonaws.elasticache.synthetic#DeleteUserOutput$ARN */ =>  {
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
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DeleteUserResult tag"))
                    };
    Ok(builder)
}

