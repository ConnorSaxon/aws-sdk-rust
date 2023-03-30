// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_delivery_streams_input(input: &crate::input::ListDeliveryStreamsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_delivery_streams_input::ser_list_delivery_streams_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_delivery_streams_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListDeliveryStreamsOutput, crate::error::ListDeliveryStreamsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListDeliveryStreamsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::ListDeliveryStreamsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_delivery_streams_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListDeliveryStreamsOutput, crate::error::ListDeliveryStreamsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_delivery_streams_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_delivery_streams::de_list_delivery_streams(response.body().as_ref(), output).map_err(crate::error::ListDeliveryStreamsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_delivery_streams(value: &[u8], mut builder: crate::output::list_delivery_streams_output::Builder) -> Result<crate::output::list_delivery_streams_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "DeliveryStreamNames" => {
                        builder = builder.set_delivery_stream_names(
                            crate::protocol_serde::shape_delivery_stream_name_list::de_delivery_stream_name_list(tokens)?
                        );
                    }
                    "HasMoreDeliveryStreams" => {
                        builder = builder.set_has_more_delivery_streams(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

