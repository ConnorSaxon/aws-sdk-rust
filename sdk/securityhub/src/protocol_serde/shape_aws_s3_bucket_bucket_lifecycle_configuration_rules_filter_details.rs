// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_s3_bucket_bucket_lifecycle_configuration_rules_filter_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsS3BucketBucketLifecycleConfigurationRulesFilterDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.predicate {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Predicate").start_object();
        crate::protocol_serde::shape_aws_s3_bucket_bucket_lifecycle_configuration_rules_filter_predicate_details::ser_aws_s3_bucket_bucket_lifecycle_configuration_rules_filter_predicate_details(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_s3_bucket_bucket_lifecycle_configuration_rules_filter_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsS3BucketBucketLifecycleConfigurationRulesFilterDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_s3_bucket_bucket_lifecycle_configuration_rules_filter_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Predicate" => {
                                builder = builder.set_predicate(
                                    crate::protocol_serde::shape_aws_s3_bucket_bucket_lifecycle_configuration_rules_filter_predicate_details::de_aws_s3_bucket_bucket_lifecycle_configuration_rules_filter_predicate_details(tokens)?
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

