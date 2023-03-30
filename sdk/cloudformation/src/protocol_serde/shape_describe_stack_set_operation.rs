// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_stack_set_operation_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeStackSetOperationOutput, crate::error::DescribeStackSetOperationError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeStackSetOperationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeStackSetOperationError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "OperationNotFoundException" => crate::error::DescribeStackSetOperationError::OperationNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_not_found_exception::de_operation_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeStackSetOperationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "StackSetNotFoundException" => crate::error::DescribeStackSetOperationError::StackSetNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::stack_set_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_stack_set_not_found_exception::de_stack_set_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeStackSetOperationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeStackSetOperationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_stack_set_operation_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeStackSetOperationOutput, crate::error::DescribeStackSetOperationError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_stack_set_operation_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_stack_set_operation::de_describe_stack_set_operation(response.body().as_ref(), output).map_err(crate::error::DescribeStackSetOperationError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_stack_set_operation(inp: &[u8], mut builder: crate::output::describe_stack_set_operation_output::Builder) -> Result<crate::output::describe_stack_set_operation_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeStackSetOperationResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeStackSetOperationResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeStackSetOperationResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeStackSetOperationResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("StackSetOperation") /* StackSetOperation com.amazonaws.cloudformation.synthetic#DescribeStackSetOperationOutput$StackSetOperation */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_stack_set_operation::de_stack_set_operation(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_stack_set_operation(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeStackSetOperationResult tag"))
                    };
    Ok(builder)
}

