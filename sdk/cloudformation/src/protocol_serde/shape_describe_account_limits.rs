// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_account_limits_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeAccountLimitsOutput, crate::error::DescribeAccountLimitsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeAccountLimitsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::DescribeAccountLimitsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_account_limits_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeAccountLimitsOutput, crate::error::DescribeAccountLimitsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_account_limits_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_account_limits::de_describe_account_limits(response.body().as_ref(), output).map_err(crate::error::DescribeAccountLimitsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_account_limits(inp: &[u8], mut builder: crate::output::describe_account_limits_output::Builder) -> Result<crate::output::describe_account_limits_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeAccountLimitsResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeAccountLimitsResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeAccountLimitsResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeAccountLimitsResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("AccountLimits") /* AccountLimits com.amazonaws.cloudformation.synthetic#DescribeAccountLimitsOutput$AccountLimits */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_account_limit_list::de_account_limit_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_account_limits(var_1);
            }
            ,
            s if s.matches("NextToken") /* NextToken com.amazonaws.cloudformation.synthetic#DescribeAccountLimitsOutput$NextToken */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_2);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeAccountLimitsResult tag"))
                    };
    Ok(builder)
}

