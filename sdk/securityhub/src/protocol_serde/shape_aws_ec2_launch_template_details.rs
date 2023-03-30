// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_ec2_launch_template_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsEc2LaunchTemplateDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.launch_template_name {
        object.key("LaunchTemplateName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.id {
        object.key("Id").string(var_2.as_str());
    }
    if let Some(var_3) = &input.launch_template_data {
        #[allow(unused_mut)]
        let mut object_4 = object.key("LaunchTemplateData").start_object();
        crate::protocol_serde::shape_aws_ec2_launch_template_data_details::ser_aws_ec2_launch_template_data_details(&mut object_4, var_3)?;
        object_4.finish();
    }
    if input.default_version_number != 0 {
        object.key("DefaultVersionNumber").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.default_version_number).into()));
    }
    if input.latest_version_number != 0 {
        object.key("LatestVersionNumber").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.latest_version_number).into()));
    }
    Ok(())
}

pub(crate) fn de_aws_ec2_launch_template_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsEc2LaunchTemplateDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_ec2_launch_template_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "LaunchTemplateName" => {
                                builder = builder.set_launch_template_name(
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
                            "LaunchTemplateData" => {
                                builder = builder.set_launch_template_data(
                                    crate::protocol_serde::shape_aws_ec2_launch_template_data_details::de_aws_ec2_launch_template_data_details(tokens)?
                                );
                            }
                            "DefaultVersionNumber" => {
                                builder = builder.set_default_version_number(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i64::try_from)
                                                        .transpose()?
                                );
                            }
                            "LatestVersionNumber" => {
                                builder = builder.set_latest_version_number(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i64::try_from)
                                                        .transpose()?
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

