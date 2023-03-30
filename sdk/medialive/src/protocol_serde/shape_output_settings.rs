// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_output_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::OutputSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.archive_output_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("archiveOutputSettings").start_object();
        crate::protocol_serde::shape_archive_output_settings::ser_archive_output_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.frame_capture_output_settings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("frameCaptureOutputSettings").start_object();
        crate::protocol_serde::shape_frame_capture_output_settings::ser_frame_capture_output_settings(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.hls_output_settings {
        #[allow(unused_mut)]
        let mut object_6 = object.key("hlsOutputSettings").start_object();
        crate::protocol_serde::shape_hls_output_settings::ser_hls_output_settings(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.media_package_output_settings {
        #[allow(unused_mut)]
        let mut object_8 = object.key("mediaPackageOutputSettings").start_object();
        crate::protocol_serde::shape_media_package_output_settings::ser_media_package_output_settings(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.ms_smooth_output_settings {
        #[allow(unused_mut)]
        let mut object_10 = object.key("msSmoothOutputSettings").start_object();
        crate::protocol_serde::shape_ms_smooth_output_settings::ser_ms_smooth_output_settings(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.multiplex_output_settings {
        #[allow(unused_mut)]
        let mut object_12 = object.key("multiplexOutputSettings").start_object();
        crate::protocol_serde::shape_multiplex_output_settings::ser_multiplex_output_settings(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.rtmp_output_settings {
        #[allow(unused_mut)]
        let mut object_14 = object.key("rtmpOutputSettings").start_object();
        crate::protocol_serde::shape_rtmp_output_settings::ser_rtmp_output_settings(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.udp_output_settings {
        #[allow(unused_mut)]
        let mut object_16 = object.key("udpOutputSettings").start_object();
        crate::protocol_serde::shape_udp_output_settings::ser_udp_output_settings(&mut object_16, var_15)?;
        object_16.finish();
    }
    Ok(())
}

pub(crate) fn de_output_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::OutputSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::output_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "archiveOutputSettings" => {
                                builder = builder.set_archive_output_settings(
                                    crate::protocol_serde::shape_archive_output_settings::de_archive_output_settings(tokens)?
                                );
                            }
                            "frameCaptureOutputSettings" => {
                                builder = builder.set_frame_capture_output_settings(
                                    crate::protocol_serde::shape_frame_capture_output_settings::de_frame_capture_output_settings(tokens)?
                                );
                            }
                            "hlsOutputSettings" => {
                                builder = builder.set_hls_output_settings(
                                    crate::protocol_serde::shape_hls_output_settings::de_hls_output_settings(tokens)?
                                );
                            }
                            "mediaPackageOutputSettings" => {
                                builder = builder.set_media_package_output_settings(
                                    crate::protocol_serde::shape_media_package_output_settings::de_media_package_output_settings(tokens)?
                                );
                            }
                            "msSmoothOutputSettings" => {
                                builder = builder.set_ms_smooth_output_settings(
                                    crate::protocol_serde::shape_ms_smooth_output_settings::de_ms_smooth_output_settings(tokens)?
                                );
                            }
                            "multiplexOutputSettings" => {
                                builder = builder.set_multiplex_output_settings(
                                    crate::protocol_serde::shape_multiplex_output_settings::de_multiplex_output_settings(tokens)?
                                );
                            }
                            "rtmpOutputSettings" => {
                                builder = builder.set_rtmp_output_settings(
                                    crate::protocol_serde::shape_rtmp_output_settings::de_rtmp_output_settings(tokens)?
                                );
                            }
                            "udpOutputSettings" => {
                                builder = builder.set_udp_output_settings(
                                    crate::protocol_serde::shape_udp_output_settings::de_udp_output_settings(tokens)?
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

