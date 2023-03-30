// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_launch_template_version_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateLaunchTemplateVersionOutput, crate::error::CreateLaunchTemplateVersionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateLaunchTemplateVersionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::CreateLaunchTemplateVersionError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_launch_template_version_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateLaunchTemplateVersionOutput, crate::error::CreateLaunchTemplateVersionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_launch_template_version_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_launch_template_version::de_create_launch_template_version(response.body().as_ref(), output).map_err(crate::error::CreateLaunchTemplateVersionError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_launch_template_version(inp: &[u8], mut builder: crate::output::create_launch_template_version_output::Builder) -> Result<crate::output::create_launch_template_version_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("CreateLaunchTemplateVersionResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected CreateLaunchTemplateVersionResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("launchTemplateVersion") /* LaunchTemplateVersion com.amazonaws.ec2.synthetic#CreateLaunchTemplateVersionOutput$LaunchTemplateVersion */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_launch_template_version::de_launch_template_version(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_launch_template_version(var_1);
            }
            ,
            s if s.matches("warning") /* Warning com.amazonaws.ec2.synthetic#CreateLaunchTemplateVersionOutput$Warning */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_validation_warning::de_validation_warning(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_warning(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

