// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_start_network_insights_analysis_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartNetworkInsightsAnalysisOutput, crate::error::StartNetworkInsightsAnalysisError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::StartNetworkInsightsAnalysisError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::StartNetworkInsightsAnalysisError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_network_insights_analysis_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartNetworkInsightsAnalysisOutput, crate::error::StartNetworkInsightsAnalysisError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_network_insights_analysis_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_start_network_insights_analysis::de_start_network_insights_analysis(response.body().as_ref(), output).map_err(crate::error::StartNetworkInsightsAnalysisError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_start_network_insights_analysis(inp: &[u8], mut builder: crate::output::start_network_insights_analysis_output::Builder) -> Result<crate::output::start_network_insights_analysis_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("StartNetworkInsightsAnalysisResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected StartNetworkInsightsAnalysisResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("networkInsightsAnalysis") /* NetworkInsightsAnalysis com.amazonaws.ec2.synthetic#StartNetworkInsightsAnalysisOutput$NetworkInsightsAnalysis */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_network_insights_analysis::de_network_insights_analysis(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_network_insights_analysis(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

