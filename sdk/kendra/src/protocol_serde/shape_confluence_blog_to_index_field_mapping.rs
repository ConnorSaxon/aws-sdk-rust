// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_confluence_blog_to_index_field_mapping(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ConfluenceBlogToIndexFieldMapping) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.data_source_field_name {
        object.key("DataSourceFieldName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.date_field_format {
        object.key("DateFieldFormat").string(var_2.as_str());
    }
    if let Some(var_3) = &input.index_field_name {
        object.key("IndexFieldName").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_confluence_blog_to_index_field_mapping<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ConfluenceBlogToIndexFieldMapping>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::confluence_blog_to_index_field_mapping::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DataSourceFieldName" => {
                                builder = builder.set_data_source_field_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ConfluenceBlogFieldName::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "DateFieldFormat" => {
                                builder = builder.set_date_field_format(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "IndexFieldName" => {
                                builder = builder.set_index_field_name(
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

