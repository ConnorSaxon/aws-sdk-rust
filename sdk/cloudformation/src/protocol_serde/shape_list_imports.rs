// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_imports_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListImportsOutput, crate::error::ListImportsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListImportsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::ListImportsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_imports_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListImportsOutput, crate::error::ListImportsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_imports_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_imports::de_list_imports(response.body().as_ref(), output).map_err(crate::error::ListImportsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_imports(inp: &[u8], mut builder: crate::output::list_imports_output::Builder) -> Result<crate::output::list_imports_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("ListImportsResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ListImportsResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("ListImportsResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected ListImportsResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Imports") /* Imports com.amazonaws.cloudformation.synthetic#ListImportsOutput$Imports */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_imports::de_imports(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_imports(var_1);
            }
            ,
            s if s.matches("NextToken") /* NextToken com.amazonaws.cloudformation.synthetic#ListImportsOutput$NextToken */ =>  {
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
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected ListImportsResult tag"))
                    };
    Ok(builder)
}

