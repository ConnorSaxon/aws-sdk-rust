// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_job_manifest(input: &crate::model::JobManifest, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.spec {
        let inner_writer = scope.start_el("Spec");
        crate::protocol_serde::shape_job_manifest_spec::ser_job_manifest_spec(var_1, inner_writer)?
    }
    if let Some(var_2) = &input.location {
        let inner_writer = scope.start_el("Location");
        crate::protocol_serde::shape_job_manifest_location::ser_job_manifest_location(var_2, inner_writer)?
    }
    scope.finish();
    Ok(())
}

pub fn de_job_manifest(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::JobManifest, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::JobManifest::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Spec") /* Spec com.amazonaws.s3control#JobManifest$Spec */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_job_manifest_spec::de_job_manifest_spec(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_spec(var_3);
            }
            ,
            s if s.matches("Location") /* Location com.amazonaws.s3control#JobManifest$Location */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_job_manifest_location::de_job_manifest_location(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_location(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

