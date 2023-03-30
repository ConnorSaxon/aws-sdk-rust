// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_policies_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribePoliciesOutput, crate::error::DescribePoliciesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribePoliciesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribePoliciesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidNextToken" => crate::error::DescribePoliciesError::InvalidNextToken({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token::de_invalid_next_token_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribePoliciesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceContention" => crate::error::DescribePoliciesError::ResourceContentionFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_contention_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_contention_fault::de_resource_contention_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribePoliciesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceLinkedRoleFailure" => crate::error::DescribePoliciesError::ServiceLinkedRoleFailure({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_linked_role_failure::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_linked_role_failure::de_service_linked_role_failure_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribePoliciesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribePoliciesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_policies_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribePoliciesOutput, crate::error::DescribePoliciesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_policies_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_policies::de_describe_policies(response.body().as_ref(), output).map_err(crate::error::DescribePoliciesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_policies(inp: &[u8], mut builder: crate::output::describe_policies_output::Builder) -> Result<crate::output::describe_policies_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribePoliciesResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribePoliciesResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribePoliciesResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribePoliciesResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("ScalingPolicies") /* ScalingPolicies com.amazonaws.autoscaling.synthetic#DescribePoliciesOutput$ScalingPolicies */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_scaling_policies::de_scaling_policies(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_scaling_policies(var_1);
            }
            ,
            s if s.matches("NextToken") /* NextToken com.amazonaws.autoscaling.synthetic#DescribePoliciesOutput$NextToken */ =>  {
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
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribePoliciesResult tag"))
                    };
    Ok(builder)
}

