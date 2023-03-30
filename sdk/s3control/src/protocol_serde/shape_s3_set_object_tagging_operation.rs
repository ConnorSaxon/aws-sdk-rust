// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_set_object_tagging_operation(input: &crate::model::S3SetObjectTaggingOperation, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.tag_set {
        let mut inner_writer = scope.start_el("TagSet").finish();
        for list_item_2 in var_1 {
             {
                let inner_writer = inner_writer.start_el("member");
                crate::protocol_serde::shape_s3_tag::ser_s3_tag(list_item_2, inner_writer)?
            }
        }
    }
    scope.finish();
    Ok(())
}

pub fn de_s3_set_object_tagging_operation(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::S3SetObjectTaggingOperation, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::S3SetObjectTaggingOperation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TagSet") /* TagSet com.amazonaws.s3control#S3SetObjectTaggingOperation$TagSet */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_s3_tag_set::de_s3_tag_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tag_set(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

