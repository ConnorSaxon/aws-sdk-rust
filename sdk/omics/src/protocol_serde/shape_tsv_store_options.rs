// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_tsv_store_options<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TsvStoreOptions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::tsv_store_options::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "annotationType" => {
                                builder = builder.set_annotation_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AnnotationType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "formatToHeader" => {
                                builder = builder.set_format_to_header(
                                    crate::protocol_serde::shape_format_to_header::de_format_to_header(tokens)?
                                );
                            }
                            "schema" => {
                                builder = builder.set_schema(
                                    crate::protocol_serde::shape_schema::de_schema(tokens)?
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

pub fn ser_tsv_store_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TsvStoreOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.annotation_type {
        object.key("annotationType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.format_to_header {
        #[allow(unused_mut)]
        let mut object_3 = object.key("formatToHeader").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.schema {
        let mut array_7 = object.key("schema").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                for (key_10, value_11) in item_8 {
                     {
                        object_9.key(key_10.as_str()).string(value_11.as_str());
                    }
                }
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}

