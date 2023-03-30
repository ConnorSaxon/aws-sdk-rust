// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_authentication(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::Authentication, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::Authentication::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Type") /* Type com.amazonaws.elasticache#Authentication$Type */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::AuthenticationType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::AuthenticationType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_1);
            }
            ,
            s if s.matches("PasswordCount") /* PasswordCount com.amazonaws.elasticache#Authentication$PasswordCount */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.elasticache#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_password_count(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

