// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_s3_storage(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::S3Storage) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AWSAccessKeyId");
    if let Some(var_2) = &input.aws_access_key_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Bucket");
    if let Some(var_4) = &input.bucket {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Prefix");
    if let Some(var_6) = &input.prefix {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("UploadPolicy");
    if let Some(var_8) = &input.upload_policy {
        scope_7.string(&aws_smithy_types::base64::encode(var_8));
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("UploadPolicySignature");
    if let Some(var_10) = &input.upload_policy_signature {
        scope_9.string(var_10);
    }
    Ok(())
}

pub fn de_s3_storage(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::S3Storage, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::S3Storage::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AWSAccessKeyId") /* AWSAccessKeyId com.amazonaws.ec2#S3Storage$AWSAccessKeyId */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_aws_access_key_id(var_11);
            }
            ,
            s if s.matches("bucket") /* Bucket com.amazonaws.ec2#S3Storage$Bucket */ =>  {
                let var_12 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket(var_12);
            }
            ,
            s if s.matches("prefix") /* Prefix com.amazonaws.ec2#S3Storage$Prefix */ =>  {
                let var_13 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_13);
            }
            ,
            s if s.matches("uploadPolicy") /* UploadPolicy com.amazonaws.ec2#S3Storage$UploadPolicy */ =>  {
                let var_14 =
                    Some(
                        aws_smithy_types::base64::decode(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        )
                        .map_err(|err|aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid base64: {:?}", err))).map(aws_smithy_types::Blob::new)
                        ?
                    )
                ;
                builder = builder.set_upload_policy(var_14);
            }
            ,
            s if s.matches("uploadPolicySignature") /* UploadPolicySignature com.amazonaws.ec2#S3Storage$UploadPolicySignature */ =>  {
                let var_15 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_upload_policy_signature(var_15);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

