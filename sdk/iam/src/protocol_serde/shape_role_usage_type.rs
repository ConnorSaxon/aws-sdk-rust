// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_role_usage_type(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::RoleUsageType, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::RoleUsageType::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Region") /* Region com.amazonaws.iam#RoleUsageType$Region */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_region(var_1);
            }
            ,
            s if s.matches("Resources") /* Resources com.amazonaws.iam#RoleUsageType$Resources */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_arn_list_type::de_arn_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_resources(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

