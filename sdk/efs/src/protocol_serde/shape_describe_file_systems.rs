// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_file_systems_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeFileSystemsOutput, crate::error::DescribeFileSystemsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeFileSystemsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeFileSystemsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequest" => crate::error::DescribeFileSystemsError::BadRequest({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request::de_bad_request_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeFileSystemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "FileSystemNotFound" => crate::error::DescribeFileSystemsError::FileSystemNotFound({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::file_system_not_found::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_file_system_not_found::de_file_system_not_found_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeFileSystemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::DescribeFileSystemsError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeFileSystemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeFileSystemsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_file_systems_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeFileSystemsOutput, crate::error::DescribeFileSystemsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_file_systems_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_file_systems::de_describe_file_systems(response.body().as_ref(), output).map_err(crate::error::DescribeFileSystemsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_file_systems(value: &[u8], mut builder: crate::output::describe_file_systems_output::Builder) -> Result<crate::output::describe_file_systems_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "FileSystems" => {
                        builder = builder.set_file_systems(
                            crate::protocol_serde::shape_file_system_descriptions::de_file_system_descriptions(tokens)?
                        );
                    }
                    "Marker" => {
                        builder = builder.set_marker(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "NextMarker" => {
                        builder = builder.set_next_marker(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
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

