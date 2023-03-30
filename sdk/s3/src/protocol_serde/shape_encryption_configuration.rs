// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_encryption_configuration(input: &crate::model::EncryptionConfiguration, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.replica_kms_key_id {
        let mut inner_writer = scope.start_el("ReplicaKmsKeyID").finish();
        inner_writer.data(
            var_1.as_str()
        );
    }
    scope.finish();
    Ok(())
}

pub fn de_encryption_configuration(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::EncryptionConfiguration, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::EncryptionConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ReplicaKmsKeyID") /* ReplicaKmsKeyID com.amazonaws.s3#EncryptionConfiguration$ReplicaKmsKeyID */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_replica_kms_key_id(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

