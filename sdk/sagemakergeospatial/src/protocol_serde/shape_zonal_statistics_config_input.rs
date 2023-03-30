// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_zonal_statistics_config_input<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ZonalStatisticsConfigInput>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::zonal_statistics_config_input::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ZoneS3Path" => {
                                builder = builder.set_zone_s3_path(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Statistics" => {
                                builder = builder.set_statistics(
                                    crate::protocol_serde::shape_zonal_statistics_list_input::de_zonal_statistics_list_input(tokens)?
                                );
                            }
                            "TargetBands" => {
                                builder = builder.set_target_bands(
                                    crate::protocol_serde::shape_string_list_input::de_string_list_input(tokens)?
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

pub fn ser_zonal_statistics_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ZonalStatisticsConfigInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.zone_s3_path {
        object.key("ZoneS3Path").string(var_1.as_str());
    }
    if let Some(var_2) = &input.statistics {
        let mut array_3 = object.key("Statistics").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.target_bands {
        let mut array_6 = object.key("TargetBands").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    Ok(())
}

