// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_search_products_input(input: &crate::input::SearchProductsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_search_products_input::ser_search_products_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_search_products_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SearchProductsOutput, crate::error::SearchProductsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::SearchProductsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::SearchProductsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParametersException" => crate::error::SearchProductsError::InvalidParametersException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameters_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameters_exception::de_invalid_parameters_exception_json_err(response.body().as_ref(), output).map_err(crate::error::SearchProductsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::SearchProductsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_search_products_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SearchProductsOutput, crate::error::SearchProductsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::search_products_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_search_products::de_search_products(response.body().as_ref(), output).map_err(crate::error::SearchProductsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_search_products(value: &[u8], mut builder: crate::output::search_products_output::Builder) -> Result<crate::output::search_products_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ProductViewSummaries" => {
                        builder = builder.set_product_view_summaries(
                            crate::protocol_serde::shape_product_view_summaries::de_product_view_summaries(tokens)?
                        );
                    }
                    "ProductViewAggregations" => {
                        builder = builder.set_product_view_aggregations(
                            crate::protocol_serde::shape_product_view_aggregations::de_product_view_aggregations(tokens)?
                        );
                    }
                    "NextPageToken" => {
                        builder = builder.set_next_page_token(
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

