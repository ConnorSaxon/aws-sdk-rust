// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_machine_learning_model_resource_data(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3MachineLearningModelResourceData) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.destination_path {
        object.key("DestinationPath").string(var_1.as_str());
    }
    if let Some(var_2) = &input.owner_setting {
        #[allow(unused_mut)]
        let mut object_3 = object.key("OwnerSetting").start_object();
        crate::protocol_serde::shape_resource_download_owner_setting::ser_resource_download_owner_setting(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.s3_uri {
        object.key("S3Uri").string(var_4.as_str());
    }
    Ok(())
}

pub(crate) fn de_s3_machine_learning_model_resource_data<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::S3MachineLearningModelResourceData>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::s3_machine_learning_model_resource_data::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DestinationPath" => {
                                builder = builder.set_destination_path(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "OwnerSetting" => {
                                builder = builder.set_owner_setting(
                                    crate::protocol_serde::shape_resource_download_owner_setting::de_resource_download_owner_setting(tokens)?
                                );
                            }
                            "S3Uri" => {
                                builder = builder.set_s3_uri(
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

