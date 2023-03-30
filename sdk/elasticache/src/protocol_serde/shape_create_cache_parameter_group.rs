// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_cache_parameter_group_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateCacheParameterGroupOutput, crate::error::CreateCacheParameterGroupError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateCacheParameterGroupError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateCacheParameterGroupError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CacheParameterGroupAlreadyExists" => crate::error::CreateCacheParameterGroupError::CacheParameterGroupAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cache_parameter_group_already_exists_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cache_parameter_group_already_exists_fault::de_cache_parameter_group_already_exists_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCacheParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CacheParameterGroupQuotaExceeded" => crate::error::CreateCacheParameterGroupError::CacheParameterGroupQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cache_parameter_group_quota_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cache_parameter_group_quota_exceeded_fault::de_cache_parameter_group_quota_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCacheParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidCacheParameterGroupState" => crate::error::CreateCacheParameterGroupError::InvalidCacheParameterGroupStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_cache_parameter_group_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_cache_parameter_group_state_fault::de_invalid_cache_parameter_group_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCacheParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterCombination" => crate::error::CreateCacheParameterGroupError::InvalidParameterCombinationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_combination_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_combination_exception::de_invalid_parameter_combination_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCacheParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValue" => crate::error::CreateCacheParameterGroupError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCacheParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TagQuotaPerResourceExceeded" => crate::error::CreateCacheParameterGroupError::TagQuotaPerResourceExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::tag_quota_per_resource_exceeded::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_tag_quota_per_resource_exceeded::de_tag_quota_per_resource_exceeded_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCacheParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateCacheParameterGroupError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_cache_parameter_group_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateCacheParameterGroupOutput, crate::error::CreateCacheParameterGroupError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_cache_parameter_group_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_cache_parameter_group::de_create_cache_parameter_group(response.body().as_ref(), output).map_err(crate::error::CreateCacheParameterGroupError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_cache_parameter_group(inp: &[u8], mut builder: crate::output::create_cache_parameter_group_output::Builder) -> Result<crate::output::create_cache_parameter_group_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("CreateCacheParameterGroupResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected CreateCacheParameterGroupResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("CreateCacheParameterGroupResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected CreateCacheParameterGroupResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("CacheParameterGroup") /* CacheParameterGroup com.amazonaws.elasticache.synthetic#CreateCacheParameterGroupOutput$CacheParameterGroup */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_cache_parameter_group::de_cache_parameter_group(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cache_parameter_group(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected CreateCacheParameterGroupResult tag"))
                    };
    Ok(builder)
}

