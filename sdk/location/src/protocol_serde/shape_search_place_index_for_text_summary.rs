// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_search_place_index_for_text_summary<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SearchPlaceIndexForTextSummary>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::search_place_index_for_text_summary::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Text" => {
                                builder = builder.set_text(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "BiasPosition" => {
                                builder = builder.set_bias_position(
                                    crate::protocol_serde::shape_position::de_position(tokens)?
                                );
                            }
                            "FilterBBox" => {
                                builder = builder.set_filter_b_box(
                                    crate::protocol_serde::shape_bounding_box::de_bounding_box(tokens)?
                                );
                            }
                            "FilterCountries" => {
                                builder = builder.set_filter_countries(
                                    crate::protocol_serde::shape_country_code_list::de_country_code_list(tokens)?
                                );
                            }
                            "MaxResults" => {
                                builder = builder.set_max_results(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "ResultBBox" => {
                                builder = builder.set_result_b_box(
                                    crate::protocol_serde::shape_bounding_box::de_bounding_box(tokens)?
                                );
                            }
                            "DataSource" => {
                                builder = builder.set_data_source(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Language" => {
                                builder = builder.set_language(
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
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

