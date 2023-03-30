// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_algorithm_specification(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AlgorithmSpecification) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.script_mode_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("scriptModeConfig").start_object();
        crate::protocol_serde::shape_script_mode_config::ser_script_mode_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.container_image {
        #[allow(unused_mut)]
        let mut object_4 = object.key("containerImage").start_object();
        crate::protocol_serde::shape_container_image::ser_container_image(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_algorithm_specification<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AlgorithmSpecification>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::algorithm_specification::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "scriptModeConfig" => {
                                builder = builder.set_script_mode_config(
                                    crate::protocol_serde::shape_script_mode_config::de_script_mode_config(tokens)?
                                );
                            }
                            "containerImage" => {
                                builder = builder.set_container_image(
                                    crate::protocol_serde::shape_container_image::de_container_image(tokens)?
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

