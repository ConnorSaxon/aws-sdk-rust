// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_s3_bucket_bucket_lifecycle_configuration_rules_noncurrent_version_transitions_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsS3BucketBucketLifecycleConfigurationRulesNoncurrentVersionTransitionsDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.days != 0 {
        object.key("Days").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.days).into()));
    }
    if let Some(var_1) = &input.storage_class {
        object.key("StorageClass").string(var_1.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_s3_bucket_bucket_lifecycle_configuration_rules_noncurrent_version_transitions_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsS3BucketBucketLifecycleConfigurationRulesNoncurrentVersionTransitionsDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_s3_bucket_bucket_lifecycle_configuration_rules_noncurrent_version_transitions_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Days" => {
                                builder = builder.set_days(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "StorageClass" => {
                                builder = builder.set_storage_class(
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

