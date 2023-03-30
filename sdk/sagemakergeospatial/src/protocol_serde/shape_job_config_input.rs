// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_job_config_input<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::JobConfigInput>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
                                Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        if variant.is_some() {
                                                            return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("encountered mixed variants in union"));
                                                        }
                        variant = match key.to_unescaped()?.as_ref() {
                            "BandMathConfig" => {
                                Some(crate::model::JobConfigInput::BandMathConfig(
                                    crate::protocol_serde::shape_band_math_config_input::de_band_math_config_input(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'BandMathConfig' cannot be null"))?
                                ))
                            }
                            "ResamplingConfig" => {
                                Some(crate::model::JobConfigInput::ResamplingConfig(
                                    crate::protocol_serde::shape_resampling_config_input::de_resampling_config_input(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ResamplingConfig' cannot be null"))?
                                ))
                            }
                            "TemporalStatisticsConfig" => {
                                Some(crate::model::JobConfigInput::TemporalStatisticsConfig(
                                    crate::protocol_serde::shape_temporal_statistics_config_input::de_temporal_statistics_config_input(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'TemporalStatisticsConfig' cannot be null"))?
                                ))
                            }
                            "CloudRemovalConfig" => {
                                Some(crate::model::JobConfigInput::CloudRemovalConfig(
                                    crate::protocol_serde::shape_cloud_removal_config_input::de_cloud_removal_config_input(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'CloudRemovalConfig' cannot be null"))?
                                ))
                            }
                            "ZonalStatisticsConfig" => {
                                Some(crate::model::JobConfigInput::ZonalStatisticsConfig(
                                    crate::protocol_serde::shape_zonal_statistics_config_input::de_zonal_statistics_config_input(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ZonalStatisticsConfig' cannot be null"))?
                                ))
                            }
                            "GeoMosaicConfig" => {
                                Some(crate::model::JobConfigInput::GeoMosaicConfig(
                                    crate::protocol_serde::shape_geo_mosaic_config_input::de_geo_mosaic_config_input(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'GeoMosaicConfig' cannot be null"))?
                                ))
                            }
                            "StackConfig" => {
                                Some(crate::model::JobConfigInput::StackConfig(
                                    crate::protocol_serde::shape_stack_config_input::de_stack_config_input(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'StackConfig' cannot be null"))?
                                ))
                            }
                            "CloudMaskingConfig" => {
                                Some(crate::model::JobConfigInput::CloudMaskingConfig(
                                    crate::protocol_serde::shape_cloud_masking_config_input::de_cloud_masking_config_input(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'CloudMaskingConfig' cannot be null"))?
                                ))
                            }
                            "LandCoverSegmentationConfig" => {
                                Some(crate::model::JobConfigInput::LandCoverSegmentationConfig(
                                    crate::protocol_serde::shape_land_cover_segmentation_config_input::de_land_cover_segmentation_config_input(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'LandCoverSegmentationConfig' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                      aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::model::JobConfigInput::Unknown)
                                                                    }
                        };
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
        }
        _ => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
    }
    Ok(variant)
}

pub fn ser_job_config_input(object_6: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::JobConfigInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::JobConfigInput::BandMathConfig(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_1 = object_6.key("BandMathConfig").start_object();
                crate::protocol_serde::shape_band_math_config_input::ser_band_math_config_input(&mut object_1, inner)?;
                object_1.finish();
            }
        },
        crate::model::JobConfigInput::ResamplingConfig(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_2 = object_6.key("ResamplingConfig").start_object();
                crate::protocol_serde::shape_resampling_config_input::ser_resampling_config_input(&mut object_2, inner)?;
                object_2.finish();
            }
        },
        crate::model::JobConfigInput::TemporalStatisticsConfig(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_3 = object_6.key("TemporalStatisticsConfig").start_object();
                crate::protocol_serde::shape_temporal_statistics_config_input::ser_temporal_statistics_config_input(&mut object_3, inner)?;
                object_3.finish();
            }
        },
        crate::model::JobConfigInput::CloudRemovalConfig(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_4 = object_6.key("CloudRemovalConfig").start_object();
                crate::protocol_serde::shape_cloud_removal_config_input::ser_cloud_removal_config_input(&mut object_4, inner)?;
                object_4.finish();
            }
        },
        crate::model::JobConfigInput::ZonalStatisticsConfig(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_5 = object_6.key("ZonalStatisticsConfig").start_object();
                crate::protocol_serde::shape_zonal_statistics_config_input::ser_zonal_statistics_config_input(&mut object_5, inner)?;
                object_5.finish();
            }
        },
        crate::model::JobConfigInput::GeoMosaicConfig(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_6 = object_6.key("GeoMosaicConfig").start_object();
                crate::protocol_serde::shape_geo_mosaic_config_input::ser_geo_mosaic_config_input(&mut object_6, inner)?;
                object_6.finish();
            }
        },
        crate::model::JobConfigInput::StackConfig(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_7 = object_6.key("StackConfig").start_object();
                crate::protocol_serde::shape_stack_config_input::ser_stack_config_input(&mut object_7, inner)?;
                object_7.finish();
            }
        },
        crate::model::JobConfigInput::CloudMaskingConfig(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_8 = object_6.key("CloudMaskingConfig").start_object();
                crate::protocol_serde::shape_cloud_masking_config_input::ser_cloud_masking_config_input(&mut object_8, inner)?;
                object_8.finish();
            }
        },
        crate::model::JobConfigInput::LandCoverSegmentationConfig(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_9 = object_6.key("LandCoverSegmentationConfig").start_object();
                crate::protocol_serde::shape_land_cover_segmentation_config_input::ser_land_cover_segmentation_config_input(&mut object_9, inner)?;
                object_9.finish();
            }
        },
        crate::model::JobConfigInput::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("JobConfigInput"))
    }
    Ok(())
}

