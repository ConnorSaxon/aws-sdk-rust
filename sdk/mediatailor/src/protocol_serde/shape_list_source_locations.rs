// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_source_locations_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListSourceLocationsOutput, crate::error::ListSourceLocationsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListSourceLocationsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::ListSourceLocationsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_source_locations_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListSourceLocationsOutput, crate::error::ListSourceLocationsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_source_locations_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_source_locations::de_list_source_locations(response.body().as_ref(), output).map_err(crate::error::ListSourceLocationsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_source_locations(value: &[u8], mut builder: crate::output::list_source_locations_output::Builder) -> Result<crate::output::list_source_locations_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Items" => {
                        builder = builder.set_items(
                            crate::protocol_serde::shape___list_of_source_location::de___list_of_source_location(tokens)?
                        );
                    }
                    "NextToken" => {
                        builder = builder.set_next_token(
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

