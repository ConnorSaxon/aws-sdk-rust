// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_json_content_types<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<std::vec::Vec<std::string::String>>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap(); break;
                    }
                    _ =>  {
                        let value =
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        ;
                        if let Some(value) = value {
                                                                    items.push(value);
                                                                }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start array or null"))
        }
    }
}

