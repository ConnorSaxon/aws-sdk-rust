// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_confluence_attachment_field_mappings_list<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<std::vec::Vec<crate::model::ConfluenceAttachmentToIndexFieldMapping>>, aws_smithy_json::deserialize::error::DeserializeError>
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
                            crate::protocol_serde::shape_confluence_attachment_to_index_field_mapping::de_confluence_attachment_to_index_field_mapping(tokens)?
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

