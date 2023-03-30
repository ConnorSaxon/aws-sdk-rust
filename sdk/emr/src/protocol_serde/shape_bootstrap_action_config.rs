// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_bootstrap_action_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BootstrapActionConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.script_bootstrap_action {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ScriptBootstrapAction").start_object();
        crate::protocol_serde::shape_script_bootstrap_action_config::ser_script_bootstrap_action_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

pub(crate) fn de_bootstrap_action_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::BootstrapActionConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::bootstrap_action_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ScriptBootstrapAction" => {
                                builder = builder.set_script_bootstrap_action(
                                    crate::protocol_serde::shape_script_bootstrap_action_config::de_script_bootstrap_action_config(tokens)?
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

