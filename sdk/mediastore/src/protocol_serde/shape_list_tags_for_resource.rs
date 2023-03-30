// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_tags_for_resource_input(input: &crate::input::ListTagsForResourceInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_tags_for_resource_input::ser_list_tags_for_resource_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_tags_for_resource_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListTagsForResourceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListTagsForResourceError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ContainerInUseException" => crate::error::ListTagsForResourceError::ContainerInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::container_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_container_in_use_exception::de_container_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ContainerNotFoundException" => crate::error::ListTagsForResourceError::ContainerNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::container_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_container_not_found_exception::de_container_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::ListTagsForResourceError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListTagsForResourceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_tags_for_resource_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_tags_for_resource_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_tags_for_resource::de_list_tags_for_resource(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_tags_for_resource(value: &[u8], mut builder: crate::output::list_tags_for_resource_output::Builder) -> Result<crate::output::list_tags_for_resource_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Tags" => {
                        builder = builder.set_tags(
                            crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?
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

