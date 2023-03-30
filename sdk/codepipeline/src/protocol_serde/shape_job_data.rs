// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_job_data<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::JobData>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::job_data::Builder::default();
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
                            "actionConfiguration" => {
                                builder = builder.set_action_configuration(
                                    crate::protocol_serde::shape_action_configuration::de_action_configuration(tokens)?
                                );
                            }
                            "pipelineContext" => {
                                builder = builder.set_pipeline_context(
                                    crate::protocol_serde::shape_pipeline_context::de_pipeline_context(tokens)?
                                );
                            }
                            "inputArtifacts" => {
                                builder = builder.set_input_artifacts(
                                    crate::protocol_serde::shape_artifact_list::de_artifact_list(tokens)?
                                );
                            }
                            "outputArtifacts" => {
                                builder = builder.set_output_artifacts(
                                    crate::protocol_serde::shape_artifact_list::de_artifact_list(tokens)?
                                );
                            }
                            "artifactCredentials" => {
                                builder = builder.set_artifact_credentials(
                                    crate::protocol_serde::shape_aws_session_credentials::de_aws_session_credentials(tokens)?
                                );
                            }
                            "continuationToken" => {
                                builder = builder.set_continuation_token(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "encryptionKey" => {
                                builder = builder.set_encryption_key(
                                    crate::protocol_serde::shape_encryption_key::de_encryption_key(tokens)?
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

