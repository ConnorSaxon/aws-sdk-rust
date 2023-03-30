// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delivery_channel(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DeliveryChannel) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.s3_bucket_name {
        object.key("s3BucketName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.s3_key_prefix {
        object.key("s3KeyPrefix").string(var_3.as_str());
    }
    if let Some(var_4) = &input.s3_kms_key_arn {
        object.key("s3KmsKeyArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.sns_topic_arn {
        object.key("snsTopicARN").string(var_5.as_str());
    }
    if let Some(var_6) = &input.config_snapshot_delivery_properties {
        #[allow(unused_mut)]
        let mut object_7 = object.key("configSnapshotDeliveryProperties").start_object();
        crate::protocol_serde::shape_config_snapshot_delivery_properties::ser_config_snapshot_delivery_properties(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}

pub(crate) fn de_delivery_channel<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DeliveryChannel>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::delivery_channel::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "s3BucketName" => {
                                builder = builder.set_s3_bucket_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "s3KeyPrefix" => {
                                builder = builder.set_s3_key_prefix(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "s3KmsKeyArn" => {
                                builder = builder.set_s3_kms_key_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "snsTopicARN" => {
                                builder = builder.set_sns_topic_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "configSnapshotDeliveryProperties" => {
                                builder = builder.set_config_snapshot_delivery_properties(
                                    crate::protocol_serde::shape_config_snapshot_delivery_properties::de_config_snapshot_delivery_properties(tokens)?
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

