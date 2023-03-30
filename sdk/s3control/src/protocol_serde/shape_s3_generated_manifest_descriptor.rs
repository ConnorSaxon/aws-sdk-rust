// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_s3_generated_manifest_descriptor(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::S3GeneratedManifestDescriptor, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::S3GeneratedManifestDescriptor::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Format") /* Format com.amazonaws.s3control#S3GeneratedManifestDescriptor$Format */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::GeneratedManifestFormat, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::GeneratedManifestFormat::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_format(var_1);
            }
            ,
            s if s.matches("Location") /* Location com.amazonaws.s3control#S3GeneratedManifestDescriptor$Location */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_job_manifest_location::de_job_manifest_location(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_location(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

