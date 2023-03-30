// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_all(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::All) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    let (_, _) = (object, input);
    Ok(())
}

pub(crate) fn de_all<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::All>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::all::Builder::default();
            aws_smithy_json::deserialize::token::skip_to_end(tokens)?;
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

