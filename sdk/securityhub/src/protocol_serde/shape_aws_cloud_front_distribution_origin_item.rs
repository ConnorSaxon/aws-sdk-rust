// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_cloud_front_distribution_origin_item(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsCloudFrontDistributionOriginItem) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain_name {
        object.key("DomainName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.id {
        object.key("Id").string(var_2.as_str());
    }
    if let Some(var_3) = &input.origin_path {
        object.key("OriginPath").string(var_3.as_str());
    }
    if let Some(var_4) = &input.s3_origin_config {
        #[allow(unused_mut)]
        let mut object_5 = object.key("S3OriginConfig").start_object();
        crate::protocol_serde::shape_aws_cloud_front_distribution_origin_s3_origin_config::ser_aws_cloud_front_distribution_origin_s3_origin_config(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.custom_origin_config {
        #[allow(unused_mut)]
        let mut object_7 = object.key("CustomOriginConfig").start_object();
        crate::protocol_serde::shape_aws_cloud_front_distribution_origin_custom_origin_config::ser_aws_cloud_front_distribution_origin_custom_origin_config(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_cloud_front_distribution_origin_item<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsCloudFrontDistributionOriginItem>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_cloud_front_distribution_origin_item::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DomainName" => {
                                builder = builder.set_domain_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Id" => {
                                builder = builder.set_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "OriginPath" => {
                                builder = builder.set_origin_path(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "S3OriginConfig" => {
                                builder = builder.set_s3_origin_config(
                                    crate::protocol_serde::shape_aws_cloud_front_distribution_origin_s3_origin_config::de_aws_cloud_front_distribution_origin_s3_origin_config(tokens)?
                                );
                            }
                            "CustomOriginConfig" => {
                                builder = builder.set_custom_origin_config(
                                    crate::protocol_serde::shape_aws_cloud_front_distribution_origin_custom_origin_config::de_aws_cloud_front_distribution_origin_custom_origin_config(tokens)?
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

