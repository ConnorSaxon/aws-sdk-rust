// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_artifact_store_map<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<std::collections::HashMap<std::string::String, crate::model::ArtifactStore>>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            let mut map = std::collections::HashMap::new();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        let key =
                            key.to_unescaped().map(|u|
                                u.into_owned()
                            )
                        ?;
                        let value =
                            crate::protocol_serde::shape_artifact_store::de_artifact_store(tokens)?
                        ;
                        if let Some(value) = value {
                                                                map.insert(key, value);
                                                            }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(map))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

