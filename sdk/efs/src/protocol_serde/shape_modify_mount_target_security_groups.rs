// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_mount_target_security_groups_input(input: &crate::input::ModifyMountTargetSecurityGroupsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_modify_mount_target_security_groups_input::ser_modify_mount_target_security_groups_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_mount_target_security_groups_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyMountTargetSecurityGroupsOutput, crate::error::ModifyMountTargetSecurityGroupsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ModifyMountTargetSecurityGroupsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ModifyMountTargetSecurityGroupsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequest" => crate::error::ModifyMountTargetSecurityGroupsError::BadRequest({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request::de_bad_request_json_err(response.body().as_ref(), output).map_err(crate::error::ModifyMountTargetSecurityGroupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IncorrectMountTargetState" => crate::error::ModifyMountTargetSecurityGroupsError::IncorrectMountTargetState({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::incorrect_mount_target_state::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_incorrect_mount_target_state::de_incorrect_mount_target_state_json_err(response.body().as_ref(), output).map_err(crate::error::ModifyMountTargetSecurityGroupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::ModifyMountTargetSecurityGroupsError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::ModifyMountTargetSecurityGroupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MountTargetNotFound" => crate::error::ModifyMountTargetSecurityGroupsError::MountTargetNotFound({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::mount_target_not_found::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_mount_target_not_found::de_mount_target_not_found_json_err(response.body().as_ref(), output).map_err(crate::error::ModifyMountTargetSecurityGroupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SecurityGroupLimitExceeded" => crate::error::ModifyMountTargetSecurityGroupsError::SecurityGroupLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::security_group_limit_exceeded::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_security_group_limit_exceeded::de_security_group_limit_exceeded_json_err(response.body().as_ref(), output).map_err(crate::error::ModifyMountTargetSecurityGroupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SecurityGroupNotFound" => crate::error::ModifyMountTargetSecurityGroupsError::SecurityGroupNotFound({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::security_group_not_found::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_security_group_not_found::de_security_group_not_found_json_err(response.body().as_ref(), output).map_err(crate::error::ModifyMountTargetSecurityGroupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ModifyMountTargetSecurityGroupsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_mount_target_security_groups_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyMountTargetSecurityGroupsOutput, crate::error::ModifyMountTargetSecurityGroupsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::modify_mount_target_security_groups_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

