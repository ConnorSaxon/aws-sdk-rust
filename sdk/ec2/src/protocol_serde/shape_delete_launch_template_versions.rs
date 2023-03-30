// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_launch_template_versions_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteLaunchTemplateVersionsOutput, crate::error::DeleteLaunchTemplateVersionsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteLaunchTemplateVersionsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::DeleteLaunchTemplateVersionsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_launch_template_versions_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteLaunchTemplateVersionsOutput, crate::error::DeleteLaunchTemplateVersionsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_launch_template_versions_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_launch_template_versions::de_delete_launch_template_versions(response.body().as_ref(), output).map_err(crate::error::DeleteLaunchTemplateVersionsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_launch_template_versions(inp: &[u8], mut builder: crate::output::delete_launch_template_versions_output::Builder) -> Result<crate::output::delete_launch_template_versions_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DeleteLaunchTemplateVersionsResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DeleteLaunchTemplateVersionsResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("successfullyDeletedLaunchTemplateVersionSet") /* SuccessfullyDeletedLaunchTemplateVersions com.amazonaws.ec2.synthetic#DeleteLaunchTemplateVersionsOutput$SuccessfullyDeletedLaunchTemplateVersions */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_delete_launch_template_versions_response_success_set::de_delete_launch_template_versions_response_success_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_successfully_deleted_launch_template_versions(var_1);
            }
            ,
            s if s.matches("unsuccessfullyDeletedLaunchTemplateVersionSet") /* UnsuccessfullyDeletedLaunchTemplateVersions com.amazonaws.ec2.synthetic#DeleteLaunchTemplateVersionsOutput$UnsuccessfullyDeletedLaunchTemplateVersions */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_delete_launch_template_versions_response_error_set::de_delete_launch_template_versions_response_error_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_unsuccessfully_deleted_launch_template_versions(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

