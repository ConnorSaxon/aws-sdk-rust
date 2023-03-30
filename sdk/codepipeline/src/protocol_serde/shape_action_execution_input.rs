// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_action_execution_input<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ActionExecutionInput>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::action_execution_input::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "actionTypeId" => {
                                builder = builder.set_action_type_id(
                                    crate::protocol_serde::shape_action_type_id::de_action_type_id(tokens)?
                                );
                            }
                            "configuration" => {
                                builder = builder.set_configuration(
                                    crate::protocol_serde::shape_action_configuration_map::de_action_configuration_map(tokens)?
                                );
                            }
                            "resolvedConfiguration" => {
                                builder = builder.set_resolved_configuration(
                                    crate::protocol_serde::shape_resolved_action_configuration_map::de_resolved_action_configuration_map(tokens)?
                                );
                            }
                            "roleArn" => {
                                builder = builder.set_role_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "region" => {
                                builder = builder.set_region(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "inputArtifacts" => {
                                builder = builder.set_input_artifacts(
                                    crate::protocol_serde::shape_artifact_detail_list::de_artifact_detail_list(tokens)?
                                );
                            }
                            "namespace" => {
                                builder = builder.set_namespace(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
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

