// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ec2_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Ec2Config) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.subnet_arn {
        object.key("SubnetArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.security_group_arns {
        let mut array_3 = object.key("SecurityGroupArns").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub(crate) fn de_ec2_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Ec2Config>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::ec2_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "SubnetArn" => {
                                builder = builder.set_subnet_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SecurityGroupArns" => {
                                builder = builder.set_security_group_arns(
                                    crate::protocol_serde::shape_ec2_security_group_arn_list::de_ec2_security_group_arn_list(tokens)?
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

