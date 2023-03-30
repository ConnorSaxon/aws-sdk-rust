// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_noncurrent_version_expiration(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::NoncurrentVersionExpiration, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::NoncurrentVersionExpiration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("NoncurrentDays") /* NoncurrentDays com.amazonaws.s3#NoncurrentVersionExpiration$NoncurrentDays */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3#Days`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_noncurrent_days(var_1);
            }
            ,
            s if s.matches("NewerNoncurrentVersions") /* NewerNoncurrentVersions com.amazonaws.s3#NoncurrentVersionExpiration$NewerNoncurrentVersions */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3#VersionCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_newer_noncurrent_versions(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

pub fn ser_noncurrent_version_expiration(input: &crate::model::NoncurrentVersionExpiration, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if input.noncurrent_days != 0 {
        let mut inner_writer = scope.start_el("NoncurrentDays").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(input.noncurrent_days).encode()
        );
    }
    if input.newer_noncurrent_versions != 0 {
        let mut inner_writer = scope.start_el("NewerNoncurrentVersions").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(input.newer_noncurrent_versions).encode()
        );
    }
    scope.finish();
    Ok(())
}

