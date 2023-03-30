// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_default_cache_behavior(input: &crate::model::DefaultCacheBehavior, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.target_origin_id {
        let mut inner_writer = scope.start_el("TargetOriginId").finish();
        inner_writer.data(
            var_1.as_str()
        );
    }
    if let Some(var_2) = &input.trusted_signers {
        let inner_writer = scope.start_el("TrustedSigners");
        crate::protocol_serde::shape_trusted_signers::ser_trusted_signers(var_2, inner_writer)?
    }
    if let Some(var_3) = &input.trusted_key_groups {
        let inner_writer = scope.start_el("TrustedKeyGroups");
        crate::protocol_serde::shape_trusted_key_groups::ser_trusted_key_groups(var_3, inner_writer)?
    }
    if let Some(var_4) = &input.viewer_protocol_policy {
        let mut inner_writer = scope.start_el("ViewerProtocolPolicy").finish();
        inner_writer.data(
            var_4.as_str()
        );
    }
    if let Some(var_5) = &input.allowed_methods {
        let inner_writer = scope.start_el("AllowedMethods");
        crate::protocol_serde::shape_allowed_methods::ser_allowed_methods(var_5, inner_writer)?
    }
    if let Some(var_6) = &input.smooth_streaming {
        let mut inner_writer = scope.start_el("SmoothStreaming").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_6).encode()
        );
    }
    if let Some(var_7) = &input.compress {
        let mut inner_writer = scope.start_el("Compress").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_7).encode()
        );
    }
    if let Some(var_8) = &input.lambda_function_associations {
        let inner_writer = scope.start_el("LambdaFunctionAssociations");
        crate::protocol_serde::shape_lambda_function_associations::ser_lambda_function_associations(var_8, inner_writer)?
    }
    if let Some(var_9) = &input.function_associations {
        let inner_writer = scope.start_el("FunctionAssociations");
        crate::protocol_serde::shape_function_associations::ser_function_associations(var_9, inner_writer)?
    }
    if let Some(var_10) = &input.field_level_encryption_id {
        let mut inner_writer = scope.start_el("FieldLevelEncryptionId").finish();
        inner_writer.data(
            var_10.as_str()
        );
    }
    if let Some(var_11) = &input.realtime_log_config_arn {
        let mut inner_writer = scope.start_el("RealtimeLogConfigArn").finish();
        inner_writer.data(
            var_11.as_str()
        );
    }
    if let Some(var_12) = &input.cache_policy_id {
        let mut inner_writer = scope.start_el("CachePolicyId").finish();
        inner_writer.data(
            var_12.as_str()
        );
    }
    if let Some(var_13) = &input.origin_request_policy_id {
        let mut inner_writer = scope.start_el("OriginRequestPolicyId").finish();
        inner_writer.data(
            var_13.as_str()
        );
    }
    if let Some(var_14) = &input.response_headers_policy_id {
        let mut inner_writer = scope.start_el("ResponseHeadersPolicyId").finish();
        inner_writer.data(
            var_14.as_str()
        );
    }
    if let Some(var_15) = &input.forwarded_values {
        let inner_writer = scope.start_el("ForwardedValues");
        crate::protocol_serde::shape_forwarded_values::ser_forwarded_values(var_15, inner_writer)?
    }
    if let Some(var_16) = &input.min_ttl {
        let mut inner_writer = scope.start_el("MinTTL").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_16).encode()
        );
    }
    if let Some(var_17) = &input.default_ttl {
        let mut inner_writer = scope.start_el("DefaultTTL").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_17).encode()
        );
    }
    if let Some(var_18) = &input.max_ttl {
        let mut inner_writer = scope.start_el("MaxTTL").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_18).encode()
        );
    }
    scope.finish();
    Ok(())
}

pub fn de_default_cache_behavior(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DefaultCacheBehavior, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DefaultCacheBehavior::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TargetOriginId") /* TargetOriginId com.amazonaws.cloudfront#DefaultCacheBehavior$TargetOriginId */ =>  {
                let var_19 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_target_origin_id(var_19);
            }
            ,
            s if s.matches("TrustedSigners") /* TrustedSigners com.amazonaws.cloudfront#DefaultCacheBehavior$TrustedSigners */ =>  {
                let var_20 =
                    Some(
                        crate::protocol_serde::shape_trusted_signers::de_trusted_signers(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_trusted_signers(var_20);
            }
            ,
            s if s.matches("TrustedKeyGroups") /* TrustedKeyGroups com.amazonaws.cloudfront#DefaultCacheBehavior$TrustedKeyGroups */ =>  {
                let var_21 =
                    Some(
                        crate::protocol_serde::shape_trusted_key_groups::de_trusted_key_groups(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_trusted_key_groups(var_21);
            }
            ,
            s if s.matches("ViewerProtocolPolicy") /* ViewerProtocolPolicy com.amazonaws.cloudfront#DefaultCacheBehavior$ViewerProtocolPolicy */ =>  {
                let var_22 =
                    Some(
                        Result::<crate::model::ViewerProtocolPolicy, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::ViewerProtocolPolicy::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_viewer_protocol_policy(var_22);
            }
            ,
            s if s.matches("AllowedMethods") /* AllowedMethods com.amazonaws.cloudfront#DefaultCacheBehavior$AllowedMethods */ =>  {
                let var_23 =
                    Some(
                        crate::protocol_serde::shape_allowed_methods::de_allowed_methods(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_allowed_methods(var_23);
            }
            ,
            s if s.matches("SmoothStreaming") /* SmoothStreaming com.amazonaws.cloudfront#DefaultCacheBehavior$SmoothStreaming */ =>  {
                let var_24 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudfront#boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_smooth_streaming(var_24);
            }
            ,
            s if s.matches("Compress") /* Compress com.amazonaws.cloudfront#DefaultCacheBehavior$Compress */ =>  {
                let var_25 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudfront#boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_compress(var_25);
            }
            ,
            s if s.matches("LambdaFunctionAssociations") /* LambdaFunctionAssociations com.amazonaws.cloudfront#DefaultCacheBehavior$LambdaFunctionAssociations */ =>  {
                let var_26 =
                    Some(
                        crate::protocol_serde::shape_lambda_function_associations::de_lambda_function_associations(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_lambda_function_associations(var_26);
            }
            ,
            s if s.matches("FunctionAssociations") /* FunctionAssociations com.amazonaws.cloudfront#DefaultCacheBehavior$FunctionAssociations */ =>  {
                let var_27 =
                    Some(
                        crate::protocol_serde::shape_function_associations::de_function_associations(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_function_associations(var_27);
            }
            ,
            s if s.matches("FieldLevelEncryptionId") /* FieldLevelEncryptionId com.amazonaws.cloudfront#DefaultCacheBehavior$FieldLevelEncryptionId */ =>  {
                let var_28 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_field_level_encryption_id(var_28);
            }
            ,
            s if s.matches("RealtimeLogConfigArn") /* RealtimeLogConfigArn com.amazonaws.cloudfront#DefaultCacheBehavior$RealtimeLogConfigArn */ =>  {
                let var_29 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_realtime_log_config_arn(var_29);
            }
            ,
            s if s.matches("CachePolicyId") /* CachePolicyId com.amazonaws.cloudfront#DefaultCacheBehavior$CachePolicyId */ =>  {
                let var_30 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cache_policy_id(var_30);
            }
            ,
            s if s.matches("OriginRequestPolicyId") /* OriginRequestPolicyId com.amazonaws.cloudfront#DefaultCacheBehavior$OriginRequestPolicyId */ =>  {
                let var_31 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_origin_request_policy_id(var_31);
            }
            ,
            s if s.matches("ResponseHeadersPolicyId") /* ResponseHeadersPolicyId com.amazonaws.cloudfront#DefaultCacheBehavior$ResponseHeadersPolicyId */ =>  {
                let var_32 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_response_headers_policy_id(var_32);
            }
            ,
            s if s.matches("ForwardedValues") /* ForwardedValues com.amazonaws.cloudfront#DefaultCacheBehavior$ForwardedValues */ =>  {
                let var_33 =
                    Some(
                        crate::protocol_serde::shape_forwarded_values::de_forwarded_values(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_forwarded_values(var_33);
            }
            ,
            s if s.matches("MinTTL") /* MinTTL com.amazonaws.cloudfront#DefaultCacheBehavior$MinTTL */ =>  {
                let var_34 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.cloudfront#long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_min_ttl(var_34);
            }
            ,
            s if s.matches("DefaultTTL") /* DefaultTTL com.amazonaws.cloudfront#DefaultCacheBehavior$DefaultTTL */ =>  {
                let var_35 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.cloudfront#long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_default_ttl(var_35);
            }
            ,
            s if s.matches("MaxTTL") /* MaxTTL com.amazonaws.cloudfront#DefaultCacheBehavior$MaxTTL */ =>  {
                let var_36 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.cloudfront#long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_ttl(var_36);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

