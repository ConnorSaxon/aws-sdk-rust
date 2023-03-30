// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_upload_ssh_public_key_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UploadSshPublicKeyOutput, crate::error::UploadSSHPublicKeyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UploadSSHPublicKeyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UploadSSHPublicKeyError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DuplicateSSHPublicKey" => crate::error::UploadSSHPublicKeyError::DuplicateSshPublicKeyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::duplicate_ssh_public_key_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_duplicate_ssh_public_key_exception::de_duplicate_ssh_public_key_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPublicKey" => crate::error::UploadSSHPublicKeyError::InvalidPublicKeyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_public_key_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_public_key_exception::de_invalid_public_key_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceeded" => crate::error::UploadSSHPublicKeyError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntity" => crate::error::UploadSSHPublicKeyError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_entity_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnrecognizedPublicKeyEncoding" => crate::error::UploadSSHPublicKeyError::UnrecognizedPublicKeyEncodingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unrecognized_public_key_encoding_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unrecognized_public_key_encoding_exception::de_unrecognized_public_key_encoding_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UploadSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UploadSSHPublicKeyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_upload_ssh_public_key_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UploadSshPublicKeyOutput, crate::error::UploadSSHPublicKeyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::upload_ssh_public_key_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_upload_ssh_public_key::de_upload_ssh_public_key(response.body().as_ref(), output).map_err(crate::error::UploadSSHPublicKeyError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_upload_ssh_public_key(inp: &[u8], mut builder: crate::output::upload_ssh_public_key_output::Builder) -> Result<crate::output::upload_ssh_public_key_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("UploadSSHPublicKeyResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected UploadSSHPublicKeyResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("UploadSSHPublicKeyResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected UploadSSHPublicKeyResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("SSHPublicKey") /* SSHPublicKey com.amazonaws.iam.synthetic#UploadSSHPublicKeyOutput$SSHPublicKey */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_ssh_public_key::de_ssh_public_key(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ssh_public_key(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected UploadSSHPublicKeyResult tag"))
                    };
    Ok(builder)
}

