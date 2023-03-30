// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lambda_action(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LambdaAction) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.function_arn {
        object.key("functionArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.payload {
        #[allow(unused_mut)]
        let mut object_3 = object.key("payload").start_object();
        crate::protocol_serde::shape_payload::ser_payload(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

pub(crate) fn de_lambda_action<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::LambdaAction>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::lambda_action::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "functionArn" => {
                                builder = builder.set_function_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "payload" => {
                                builder = builder.set_payload(
                                    crate::protocol_serde::shape_payload::de_payload(tokens)?
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

