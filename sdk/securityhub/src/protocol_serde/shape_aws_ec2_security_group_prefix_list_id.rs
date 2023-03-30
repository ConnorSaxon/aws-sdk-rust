// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_ec2_security_group_prefix_list_id(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsEc2SecurityGroupPrefixListId) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.prefix_list_id {
        object.key("PrefixListId").string(var_1.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_ec2_security_group_prefix_list_id<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsEc2SecurityGroupPrefixListId>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_ec2_security_group_prefix_list_id::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "PrefixListId" => {
                                builder = builder.set_prefix_list_id(
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

