// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_bundle_task_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CancelBundleTaskOutput, crate::error::CancelBundleTaskError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CancelBundleTaskError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::CancelBundleTaskError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_bundle_task_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CancelBundleTaskOutput, crate::error::CancelBundleTaskError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::cancel_bundle_task_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_cancel_bundle_task::de_cancel_bundle_task(response.body().as_ref(), output).map_err(crate::error::CancelBundleTaskError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_cancel_bundle_task(inp: &[u8], mut builder: crate::output::cancel_bundle_task_output::Builder) -> Result<crate::output::cancel_bundle_task_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("CancelBundleTaskResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected CancelBundleTaskResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("bundleInstanceTask") /* BundleTask com.amazonaws.ec2.synthetic#CancelBundleTaskOutput$BundleTask */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_bundle_task::de_bundle_task(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_bundle_task(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

