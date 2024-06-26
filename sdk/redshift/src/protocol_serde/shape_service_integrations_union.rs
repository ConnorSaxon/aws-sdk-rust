// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_service_integrations_union(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::ServiceIntegrationsUnion,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::ServiceIntegrationsUnion::LakeFormation(inner) => {
            let mut list_2 = writer.start_list(false, None);
            for item_1 in inner {
                #[allow(unused_mut)]
                let mut entry_3 = list_2.entry();
                #[allow(unused_mut)]
                let mut scope_4 = entry_3.prefix("member");
                crate::protocol_serde::shape_lake_formation_scope_union::ser_lake_formation_scope_union(scope_4, item_1)?;
            }
            list_2.finish();
        }
        crate::types::ServiceIntegrationsUnion::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "ServiceIntegrationsUnion",
            ))
        }
    }
    Ok(())
}

pub fn de_service_integrations_union(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ServiceIntegrationsUnion, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut base: Option<crate::types::ServiceIntegrationsUnion> = None;
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("LakeFormation") /* LakeFormation com.amazonaws.redshift#ServiceIntegrationsUnion$LakeFormation */ =>  {
                let tmp =
                    crate::protocol_serde::shape_lake_formation_service_integrations::de_lake_formation_service_integrations(&mut tag)
                    ?
                ;
                base = Some(crate::types::ServiceIntegrationsUnion::LakeFormation(tmp));
            }
            ,
            _unknown => base = Some(crate::types::ServiceIntegrationsUnion::Unknown),
        }
    }
    base.ok_or_else(|| ::aws_smithy_xml::decode::XmlDecodeError::custom("expected union, got nothing"))
}
