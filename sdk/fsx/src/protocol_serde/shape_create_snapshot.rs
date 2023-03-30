// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_snapshot_input(input: &crate::input::CreateSnapshotInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_snapshot_input::ser_create_snapshot_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_snapshot_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateSnapshotOutput, crate::error::CreateSnapshotError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateSnapshotError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateSnapshotError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequest" => crate::error::CreateSnapshotError::BadRequest({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request::de_bad_request_json_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::CreateSnapshotError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceLimitExceeded" => crate::error::CreateSnapshotError::ServiceLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_limit_exceeded::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_limit_exceeded::de_service_limit_exceeded_json_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "VolumeNotFound" => crate::error::CreateSnapshotError::VolumeNotFound({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::volume_not_found::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_volume_not_found::de_volume_not_found_json_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateSnapshotError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_snapshot_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateSnapshotOutput, crate::error::CreateSnapshotError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_snapshot_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_snapshot::de_create_snapshot(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_snapshot(value: &[u8], mut builder: crate::output::create_snapshot_output::Builder) -> Result<crate::output::create_snapshot_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Snapshot" => {
                        builder = builder.set_snapshot(
                            crate::protocol_serde::shape_snapshot::de_snapshot(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

