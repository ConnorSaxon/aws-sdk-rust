// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_aws_s3_bucket_website_configuration_routing_rules<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<std::vec::Vec<crate::model::AwsS3BucketWebsiteConfigurationRoutingRule>>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap(); break;
                    }
                    _ =>  {
                        let value =
                            crate::protocol_serde::shape_aws_s3_bucket_website_configuration_routing_rule::de_aws_s3_bucket_website_configuration_routing_rule(tokens)?
                        ;
                        if let Some(value) = value {
                                                                    items.push(value);
                                                                }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start array or null"))
        }
    }
}

