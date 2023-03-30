// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_coverage<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Coverage>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::coverage::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "CoverageHours" => {
                                builder = builder.set_coverage_hours(
                                    crate::protocol_serde::shape_coverage_hours::de_coverage_hours(tokens)?
                                );
                            }
                            "CoverageNormalizedUnits" => {
                                builder = builder.set_coverage_normalized_units(
                                    crate::protocol_serde::shape_coverage_normalized_units::de_coverage_normalized_units(tokens)?
                                );
                            }
                            "CoverageCost" => {
                                builder = builder.set_coverage_cost(
                                    crate::protocol_serde::shape_coverage_cost::de_coverage_cost(tokens)?
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

