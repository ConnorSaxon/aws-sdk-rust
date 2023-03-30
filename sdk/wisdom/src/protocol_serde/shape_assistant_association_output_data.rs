// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_assistant_association_output_data<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AssistantAssociationOutputData>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
                                Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        if variant.is_some() {
                                                            return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("encountered mixed variants in union"));
                                                        }
                        variant = match key.to_unescaped()?.as_ref() {
                            "knowledgeBaseAssociation" => {
                                Some(crate::model::AssistantAssociationOutputData::KnowledgeBaseAssociation(
                                    crate::protocol_serde::shape_knowledge_base_association_data::de_knowledge_base_association_data(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'knowledgeBaseAssociation' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                      aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::model::AssistantAssociationOutputData::Unknown)
                                                                    }
                        };
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
        }
        _ => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
    }
    Ok(variant)
}

