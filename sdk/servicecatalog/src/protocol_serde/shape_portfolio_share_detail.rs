// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_portfolio_share_detail<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PortfolioShareDetail>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::portfolio_share_detail::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "PrincipalId" => {
                                builder = builder.set_principal_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Type" => {
                                builder = builder.set_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::DescribePortfolioShareType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Accepted" => {
                                builder = builder.set_accepted(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "ShareTagOptions" => {
                                builder = builder.set_share_tag_options(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "SharePrincipals" => {
                                builder = builder.set_share_principals(
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

