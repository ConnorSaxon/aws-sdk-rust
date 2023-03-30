// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_s3_account_public_access_block_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsS3AccountPublicAccessBlockDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.block_public_acls {
        object.key("BlockPublicAcls").boolean(input.block_public_acls);
    }
    if input.block_public_policy {
        object.key("BlockPublicPolicy").boolean(input.block_public_policy);
    }
    if input.ignore_public_acls {
        object.key("IgnorePublicAcls").boolean(input.ignore_public_acls);
    }
    if input.restrict_public_buckets {
        object.key("RestrictPublicBuckets").boolean(input.restrict_public_buckets);
    }
    Ok(())
}

pub(crate) fn de_aws_s3_account_public_access_block_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsS3AccountPublicAccessBlockDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_s3_account_public_access_block_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "BlockPublicAcls" => {
                                builder = builder.set_block_public_acls(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "BlockPublicPolicy" => {
                                builder = builder.set_block_public_policy(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "IgnorePublicAcls" => {
                                builder = builder.set_ignore_public_acls(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "RestrictPublicBuckets" => {
                                builder = builder.set_restrict_public_buckets(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

