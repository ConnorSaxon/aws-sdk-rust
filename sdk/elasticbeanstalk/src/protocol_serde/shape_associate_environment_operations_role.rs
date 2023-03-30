// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_environment_operations_role_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AssociateEnvironmentOperationsRoleOutput, crate::error::AssociateEnvironmentOperationsRoleError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AssociateEnvironmentOperationsRoleError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::AssociateEnvironmentOperationsRoleError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InsufficientPrivilegesException" => crate::error::AssociateEnvironmentOperationsRoleError::InsufficientPrivilegesException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::insufficient_privileges_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_privileges_exception::de_insufficient_privileges_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::AssociateEnvironmentOperationsRoleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::AssociateEnvironmentOperationsRoleError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_environment_operations_role_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AssociateEnvironmentOperationsRoleOutput, crate::error::AssociateEnvironmentOperationsRoleError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::associate_environment_operations_role_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

