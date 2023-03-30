// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_ssh_public_key_metadata(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::SshPublicKeyMetadata, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::SshPublicKeyMetadata::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("UserName") /* UserName com.amazonaws.iam#SSHPublicKeyMetadata$UserName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_user_name(var_1);
            }
            ,
            s if s.matches("SSHPublicKeyId") /* SSHPublicKeyId com.amazonaws.iam#SSHPublicKeyMetadata$SSHPublicKeyId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ssh_public_key_id(var_2);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.iam#SSHPublicKeyMetadata$Status */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::model::StatusType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::StatusType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_3);
            }
            ,
            s if s.matches("UploadDate") /* UploadDate com.amazonaws.iam#SSHPublicKeyMetadata$UploadDate */ =>  {
                let var_4 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.iam#dateType`)"))
                        ?
                    )
                ;
                builder = builder.set_upload_date(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

