// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_scheduled_actions_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeScheduledActionsOutput, crate::error::DescribeScheduledActionsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeScheduledActionsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeScheduledActionsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ScheduledActionNotFound" => crate::error::DescribeScheduledActionsError::ScheduledActionNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::scheduled_action_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_scheduled_action_not_found_fault::de_scheduled_action_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeScheduledActionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedOperation" => crate::error::DescribeScheduledActionsError::UnauthorizedOperation({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_operation::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_operation::de_unauthorized_operation_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeScheduledActionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeScheduledActionsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_scheduled_actions_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeScheduledActionsOutput, crate::error::DescribeScheduledActionsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_scheduled_actions_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_scheduled_actions::de_describe_scheduled_actions(response.body().as_ref(), output).map_err(crate::error::DescribeScheduledActionsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_scheduled_actions(inp: &[u8], mut builder: crate::output::describe_scheduled_actions_output::Builder) -> Result<crate::output::describe_scheduled_actions_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeScheduledActionsResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeScheduledActionsResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeScheduledActionsResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeScheduledActionsResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Marker") /* Marker com.amazonaws.redshift.synthetic#DescribeScheduledActionsOutput$Marker */ =>  {
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
            s if s.matches("ScheduledActions") /* ScheduledActions com.amazonaws.redshift.synthetic#DescribeScheduledActionsOutput$ScheduledActions */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_scheduled_action_list::de_scheduled_action_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_scheduled_actions(var_2);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeScheduledActionsResult tag"))
                    };
    Ok(builder)
}

