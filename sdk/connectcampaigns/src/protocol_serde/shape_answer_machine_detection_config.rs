// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_answer_machine_detection_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AnswerMachineDetectionConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.enable_answer_machine_detection {
        object.key("enableAnswerMachineDetection").boolean(*var_1);
    }
    Ok(())
}

pub(crate) fn de_answer_machine_detection_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AnswerMachineDetectionConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::answer_machine_detection_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "enableAnswerMachineDetection" => {
                                builder = builder.set_enable_answer_machine_detection(
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

