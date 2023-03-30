// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_identity_dkim_attributes(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::IdentityDkimAttributes, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::IdentityDkimAttributes::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DkimEnabled") /* DkimEnabled com.amazonaws.ses#IdentityDkimAttributes$DkimEnabled */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ses#Enabled`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_dkim_enabled(var_1);
            }
            ,
            s if s.matches("DkimVerificationStatus") /* DkimVerificationStatus com.amazonaws.ses#IdentityDkimAttributes$DkimVerificationStatus */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::model::VerificationStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::VerificationStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_dkim_verification_status(var_2);
            }
            ,
            s if s.matches("DkimTokens") /* DkimTokens com.amazonaws.ses#IdentityDkimAttributes$DkimTokens */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_verification_token_list::de_verification_token_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_dkim_tokens(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

