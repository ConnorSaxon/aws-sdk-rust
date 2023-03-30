// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_spot_fleet_tag_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::SpotFleetTagSpecification) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ResourceType");
    if let Some(var_2) = &input.resource_type {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Tag");
    if let Some(var_4) = &input.tags {
        let mut list_6 = scope_3.start_list(true, Some("item"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_7, item_5)?;
        }
        list_6.finish();
    }
    Ok(())
}

pub fn de_spot_fleet_tag_specification(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::SpotFleetTagSpecification, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::SpotFleetTagSpecification::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("resourceType") /* ResourceType com.amazonaws.ec2#SpotFleetTagSpecification$ResourceType */ =>  {
                let var_8 =
                    Some(
                        Result::<crate::model::ResourceType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::ResourceType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_type(var_8);
            }
            ,
            s if s.matches("tag") /* Tags com.amazonaws.ec2#SpotFleetTagSpecification$Tags */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

