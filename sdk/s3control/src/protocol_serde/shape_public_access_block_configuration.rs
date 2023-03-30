// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_public_access_block_configuration(input: &crate::model::PublicAccessBlockConfiguration, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if input.block_public_acls {
        let mut inner_writer = scope.start_el("BlockPublicAcls").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(input.block_public_acls).encode()
        );
    }
    if input.ignore_public_acls {
        let mut inner_writer = scope.start_el("IgnorePublicAcls").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(input.ignore_public_acls).encode()
        );
    }
    if input.block_public_policy {
        let mut inner_writer = scope.start_el("BlockPublicPolicy").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(input.block_public_policy).encode()
        );
    }
    if input.restrict_public_buckets {
        let mut inner_writer = scope.start_el("RestrictPublicBuckets").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(input.restrict_public_buckets).encode()
        );
    }
    scope.finish();
    Ok(())
}

pub fn de_public_access_block_configuration(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::PublicAccessBlockConfiguration, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::PublicAccessBlockConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("BlockPublicAcls") /* BlockPublicAcls com.amazonaws.s3control#PublicAccessBlockConfiguration$BlockPublicAcls */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3control#Setting`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_block_public_acls(var_1);
            }
            ,
            s if s.matches("IgnorePublicAcls") /* IgnorePublicAcls com.amazonaws.s3control#PublicAccessBlockConfiguration$IgnorePublicAcls */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3control#Setting`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_ignore_public_acls(var_2);
            }
            ,
            s if s.matches("BlockPublicPolicy") /* BlockPublicPolicy com.amazonaws.s3control#PublicAccessBlockConfiguration$BlockPublicPolicy */ =>  {
                let var_3 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3control#Setting`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_block_public_policy(var_3);
            }
            ,
            s if s.matches("RestrictPublicBuckets") /* RestrictPublicBuckets com.amazonaws.s3control#PublicAccessBlockConfiguration$RestrictPublicBuckets */ =>  {
                let var_4 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3control#Setting`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_restrict_public_buckets(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

