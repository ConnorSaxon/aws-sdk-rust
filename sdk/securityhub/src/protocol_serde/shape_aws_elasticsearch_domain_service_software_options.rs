// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_elasticsearch_domain_service_software_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsElasticsearchDomainServiceSoftwareOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.automated_update_date {
        object.key("AutomatedUpdateDate").string(var_1.as_str());
    }
    if input.cancellable {
        object.key("Cancellable").boolean(input.cancellable);
    }
    if let Some(var_2) = &input.current_version {
        object.key("CurrentVersion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.new_version {
        object.key("NewVersion").string(var_4.as_str());
    }
    if input.update_available {
        object.key("UpdateAvailable").boolean(input.update_available);
    }
    if let Some(var_5) = &input.update_status {
        object.key("UpdateStatus").string(var_5.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_elasticsearch_domain_service_software_options<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsElasticsearchDomainServiceSoftwareOptions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_elasticsearch_domain_service_software_options::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AutomatedUpdateDate" => {
                                builder = builder.set_automated_update_date(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Cancellable" => {
                                builder = builder.set_cancellable(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CurrentVersion" => {
                                builder = builder.set_current_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "NewVersion" => {
                                builder = builder.set_new_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "UpdateAvailable" => {
                                builder = builder.set_update_available(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "UpdateStatus" => {
                                builder = builder.set_update_status(
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

