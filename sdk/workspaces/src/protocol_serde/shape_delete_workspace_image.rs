// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_workspace_image_input(input: &crate::input::DeleteWorkspaceImageInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_workspace_image_input::ser_delete_workspace_image_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_workspace_image_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteWorkspaceImageOutput, crate::error::DeleteWorkspaceImageError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteWorkspaceImageError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteWorkspaceImageError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DeleteWorkspaceImageError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteWorkspaceImageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidResourceStateException" => crate::error::DeleteWorkspaceImageError::InvalidResourceStateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_resource_state_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_resource_state_exception::de_invalid_resource_state_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteWorkspaceImageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceAssociatedException" => crate::error::DeleteWorkspaceImageError::ResourceAssociatedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_associated_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_associated_exception::de_resource_associated_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteWorkspaceImageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteWorkspaceImageError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_workspace_image_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteWorkspaceImageOutput, crate::error::DeleteWorkspaceImageError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_workspace_image_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

