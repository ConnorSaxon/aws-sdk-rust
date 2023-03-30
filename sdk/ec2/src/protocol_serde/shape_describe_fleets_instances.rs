// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_describe_fleets_instances(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DescribeFleetsInstances, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DescribeFleetsInstances::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("launchTemplateAndOverrides") /* LaunchTemplateAndOverrides com.amazonaws.ec2#DescribeFleetsInstances$LaunchTemplateAndOverrides */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_launch_template_and_overrides_response::de_launch_template_and_overrides_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_launch_template_and_overrides(var_1);
            }
            ,
            s if s.matches("lifecycle") /* Lifecycle com.amazonaws.ec2#DescribeFleetsInstances$Lifecycle */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::model::InstanceLifecycle, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::InstanceLifecycle::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_lifecycle(var_2);
            }
            ,
            s if s.matches("instanceIds") /* InstanceIds com.amazonaws.ec2#DescribeFleetsInstances$InstanceIds */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_instance_ids_set::de_instance_ids_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_ids(var_3);
            }
            ,
            s if s.matches("instanceType") /* InstanceType com.amazonaws.ec2#DescribeFleetsInstances$InstanceType */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::model::InstanceType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::InstanceType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_type(var_4);
            }
            ,
            s if s.matches("platform") /* Platform com.amazonaws.ec2#DescribeFleetsInstances$Platform */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::model::PlatformValues, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::PlatformValues::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_platform(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

