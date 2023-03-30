// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_cache_engine_versions_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeCacheEngineVersionsOutput, crate::error::DescribeCacheEngineVersionsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeCacheEngineVersionsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::DescribeCacheEngineVersionsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_cache_engine_versions_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeCacheEngineVersionsOutput, crate::error::DescribeCacheEngineVersionsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_cache_engine_versions_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_cache_engine_versions::de_describe_cache_engine_versions(response.body().as_ref(), output).map_err(crate::error::DescribeCacheEngineVersionsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_cache_engine_versions(inp: &[u8], mut builder: crate::output::describe_cache_engine_versions_output::Builder) -> Result<crate::output::describe_cache_engine_versions_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeCacheEngineVersionsResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeCacheEngineVersionsResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeCacheEngineVersionsResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeCacheEngineVersionsResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Marker") /* Marker com.amazonaws.elasticache.synthetic#DescribeCacheEngineVersionsOutput$Marker */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_marker(var_1);
            }
            ,
            s if s.matches("CacheEngineVersions") /* CacheEngineVersions com.amazonaws.elasticache.synthetic#DescribeCacheEngineVersionsOutput$CacheEngineVersions */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_cache_engine_version_list::de_cache_engine_version_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cache_engine_versions(var_2);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeCacheEngineVersionsResult tag"))
                    };
    Ok(builder)
}

