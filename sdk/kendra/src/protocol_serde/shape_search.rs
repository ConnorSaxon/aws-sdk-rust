// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_search(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Search) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.facetable {
        object.key("Facetable").boolean(input.facetable);
    }
    if input.searchable {
        object.key("Searchable").boolean(input.searchable);
    }
    if input.displayable {
        object.key("Displayable").boolean(input.displayable);
    }
    if input.sortable {
        object.key("Sortable").boolean(input.sortable);
    }
    Ok(())
}

pub(crate) fn de_search<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Search>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::search::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Facetable" => {
                                builder = builder.set_facetable(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "Searchable" => {
                                builder = builder.set_searchable(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "Displayable" => {
                                builder = builder.set_displayable(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "Sortable" => {
                                builder = builder.set_sortable(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

