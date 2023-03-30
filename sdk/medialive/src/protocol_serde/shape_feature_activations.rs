// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_feature_activations(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FeatureActivations) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.input_prepare_schedule_actions {
        object.key("inputPrepareScheduleActions").string(var_1.as_str());
    }
    Ok(())
}

pub(crate) fn de_feature_activations<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::FeatureActivations>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::feature_activations::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "inputPrepareScheduleActions" => {
                                builder = builder.set_input_prepare_schedule_actions(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::FeatureActivationsInputPrepareScheduleActions::from(u.as_ref())
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

