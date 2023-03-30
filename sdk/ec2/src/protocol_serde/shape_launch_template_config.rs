// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_launch_template_config(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::LaunchTemplateConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LaunchTemplateSpecification");
    if let Some(var_2) = &input.launch_template_specification {
        crate::protocol_serde::shape_fleet_launch_template_specification::ser_fleet_launch_template_specification(scope_1, var_2)?;
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Overrides");
    if let Some(var_4) = &input.overrides {
        let mut list_6 = scope_3.start_list(true, Some("item"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            crate::protocol_serde::shape_launch_template_overrides::ser_launch_template_overrides(entry_7, item_5)?;
        }
        list_6.finish();
    }
    Ok(())
}

pub fn de_launch_template_config(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::LaunchTemplateConfig, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::LaunchTemplateConfig::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("launchTemplateSpecification") /* LaunchTemplateSpecification com.amazonaws.ec2#LaunchTemplateConfig$LaunchTemplateSpecification */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_fleet_launch_template_specification::de_fleet_launch_template_specification(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_launch_template_specification(var_8);
            }
            ,
            s if s.matches("overrides") /* Overrides com.amazonaws.ec2#LaunchTemplateConfig$Overrides */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_launch_template_overrides_list::de_launch_template_overrides_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_overrides(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

