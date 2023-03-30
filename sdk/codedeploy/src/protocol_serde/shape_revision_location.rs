// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_revision_location(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RevisionLocation) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.revision_type {
        object.key("revisionType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.s3_location {
        #[allow(unused_mut)]
        let mut object_3 = object.key("s3Location").start_object();
        crate::protocol_serde::shape_s3_location::ser_s3_location(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.git_hub_location {
        #[allow(unused_mut)]
        let mut object_5 = object.key("gitHubLocation").start_object();
        crate::protocol_serde::shape_git_hub_location::ser_git_hub_location(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.string {
        #[allow(unused_mut)]
        let mut object_7 = object.key("string").start_object();
        crate::protocol_serde::shape_raw_string::ser_raw_string(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.app_spec_content {
        #[allow(unused_mut)]
        let mut object_9 = object.key("appSpecContent").start_object();
        crate::protocol_serde::shape_app_spec_content::ser_app_spec_content(&mut object_9, var_8)?;
        object_9.finish();
    }
    Ok(())
}

pub(crate) fn de_revision_location<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RevisionLocation>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::revision_location::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "revisionType" => {
                                builder = builder.set_revision_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::RevisionLocationType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "s3Location" => {
                                builder = builder.set_s3_location(
                                    crate::protocol_serde::shape_s3_location::de_s3_location(tokens)?
                                );
                            }
                            "gitHubLocation" => {
                                builder = builder.set_git_hub_location(
                                    crate::protocol_serde::shape_git_hub_location::de_git_hub_location(tokens)?
                                );
                            }
                            "string" => {
                                builder = builder.set_string(
                                    crate::protocol_serde::shape_raw_string::de_raw_string(tokens)?
                                );
                            }
                            "appSpecContent" => {
                                builder = builder.set_app_spec_content(
                                    crate::protocol_serde::shape_app_spec_content::de_app_spec_content(tokens)?
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

