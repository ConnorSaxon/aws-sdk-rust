// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_dialog_code_hook_invocation_setting<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DialogCodeHookInvocationSetting>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::dialog_code_hook_invocation_setting::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "enableCodeHookInvocation" => {
                                builder = builder.set_enable_code_hook_invocation(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "active" => {
                                builder = builder.set_active(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "invocationLabel" => {
                                builder = builder.set_invocation_label(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "postCodeHookSpecification" => {
                                builder = builder.set_post_code_hook_specification(
                                    crate::protocol_serde::shape_post_dialog_code_hook_invocation_specification::de_post_dialog_code_hook_invocation_specification(tokens)?
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

pub fn ser_dialog_code_hook_invocation_setting(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DialogCodeHookInvocationSetting) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.enable_code_hook_invocation {
        object.key("enableCodeHookInvocation").boolean(*var_1);
    }
    if let Some(var_2) = &input.active {
        object.key("active").boolean(*var_2);
    }
    if let Some(var_3) = &input.invocation_label {
        object.key("invocationLabel").string(var_3.as_str());
    }
    if let Some(var_4) = &input.post_code_hook_specification {
        #[allow(unused_mut)]
        let mut object_5 = object.key("postCodeHookSpecification").start_object();
        crate::protocol_serde::shape_post_dialog_code_hook_invocation_specification::ser_post_dialog_code_hook_invocation_specification(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

