// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_available_solution_stacks_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListAvailableSolutionStacksOutput, crate::error::ListAvailableSolutionStacksError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListAvailableSolutionStacksError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::ListAvailableSolutionStacksError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_available_solution_stacks_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListAvailableSolutionStacksOutput, crate::error::ListAvailableSolutionStacksError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_available_solution_stacks_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_available_solution_stacks::de_list_available_solution_stacks(response.body().as_ref(), output).map_err(crate::error::ListAvailableSolutionStacksError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_available_solution_stacks(inp: &[u8], mut builder: crate::output::list_available_solution_stacks_output::Builder) -> Result<crate::output::list_available_solution_stacks_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("ListAvailableSolutionStacksResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ListAvailableSolutionStacksResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("ListAvailableSolutionStacksResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected ListAvailableSolutionStacksResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("SolutionStacks") /* SolutionStacks com.amazonaws.elasticbeanstalk.synthetic#ListAvailableSolutionStacksOutput$SolutionStacks */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_available_solution_stack_names_list::de_available_solution_stack_names_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_solution_stacks(var_1);
            }
            ,
            s if s.matches("SolutionStackDetails") /* SolutionStackDetails com.amazonaws.elasticbeanstalk.synthetic#ListAvailableSolutionStacksOutput$SolutionStackDetails */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_available_solution_stack_details_list::de_available_solution_stack_details_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_solution_stack_details(var_2);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected ListAvailableSolutionStacksResult tag"))
                    };
    Ok(builder)
}

