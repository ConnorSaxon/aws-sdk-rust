// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateSiteRackPhysicalProperties`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`site_id(impl Into<String>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::site_id) / [`set_site_id(Option<String>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::set_site_id): <p> The ID or the Amazon Resource Name (ARN) of the site. </p>
    ///   - [`power_draw_kva(PowerDrawKva)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::power_draw_kva) / [`set_power_draw_kva(Option<PowerDrawKva>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::set_power_draw_kva): <p>The power draw, in kVA, available at the hardware placement position for the rack.</p>
    ///   - [`power_phase(PowerPhase)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::power_phase) / [`set_power_phase(Option<PowerPhase>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::set_power_phase): <p>The power option that you can provide for hardware. </p>  <ul>   <li> <p>Single-phase AC feed: 200 V to 277 V, 50 Hz or 60 Hz</p> </li>   <li> <p>Three-phase AC feed: 346 V to 480 V, 50 Hz or 60 Hz</p> </li>  </ul>
    ///   - [`power_connector(PowerConnector)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::power_connector) / [`set_power_connector(Option<PowerConnector>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::set_power_connector): <p>The power connector that Amazon Web Services should plan to provide for connections to the hardware. Note the correlation between <code>PowerPhase</code> and <code>PowerConnector</code>. </p>  <ul>   <li> <p>Single-phase AC feed</p>    <ul>     <li> <p> <b>L6-30P</b> – (common in US); 30A; single phase</p> </li>     <li> <p> <b>IEC309 (blue)</b> – P+N+E, 6hr; 32 A; single phase</p> </li>    </ul> </li>   <li> <p>Three-phase AC feed</p>    <ul>     <li> <p> <b>AH530P7W (red)</b> – 3P+N+E, 7hr; 30A; three phase</p> </li>     <li> <p> <b>AH532P6W (red)</b> – 3P+N+E, 6hr; 32A; three phase</p> </li>    </ul> </li>  </ul>
    ///   - [`power_feed_drop(PowerFeedDrop)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::power_feed_drop) / [`set_power_feed_drop(Option<PowerFeedDrop>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::set_power_feed_drop): <p>Indicates whether the power feed comes above or below the rack. </p>
    ///   - [`uplink_gbps(UplinkGbps)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::uplink_gbps) / [`set_uplink_gbps(Option<UplinkGbps>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::set_uplink_gbps): <p>The uplink speed the rack should support for the connection to the Region. </p>
    ///   - [`uplink_count(UplinkCount)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::uplink_count) / [`set_uplink_count(Option<UplinkCount>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::set_uplink_count): <p>Racks come with two Outpost network devices. Depending on the supported uplink speed at the site, the Outpost network devices provide a variable number of uplinks. Specify the number of uplinks for each Outpost network device that you intend to use to connect the rack to your network. Note the correlation between <code>UplinkGbps</code> and <code>UplinkCount</code>. </p>  <ul>   <li> <p>1Gbps - Uplinks available: 1, 2, 4, 6, 8</p> </li>   <li> <p>10Gbps - Uplinks available: 1, 2, 4, 8, 12, 16</p> </li>   <li> <p>40 and 100 Gbps- Uplinks available: 1, 2, 4</p> </li>  </ul>
    ///   - [`fiber_optic_cable_type(FiberOpticCableType)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::fiber_optic_cable_type) / [`set_fiber_optic_cable_type(Option<FiberOpticCableType>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::set_fiber_optic_cable_type): <p>The type of fiber that you will use to attach the Outpost to your network. </p>
    ///   - [`optical_standard(OpticalStandard)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::optical_standard) / [`set_optical_standard(Option<OpticalStandard>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::set_optical_standard): <p>The type of optical standard that you will use to attach the Outpost to your network. This field is dependent on uplink speed, fiber type, and distance to the upstream device. For more information about networking requirements for racks, see <a href="https://docs.aws.amazon.com/outposts/latest/userguide/outposts-requirements.html#facility-networking">Network</a> in the Amazon Web Services Outposts User Guide. </p>  <ul>   <li> <p> <code>OPTIC_10GBASE_SR</code>: 10GBASE-SR</p> </li>   <li> <p> <code>OPTIC_10GBASE_IR</code>: 10GBASE-IR</p> </li>   <li> <p> <code>OPTIC_10GBASE_LR</code>: 10GBASE-LR</p> </li>   <li> <p> <code>OPTIC_40GBASE_SR</code>: 40GBASE-SR</p> </li>   <li> <p> <code>OPTIC_40GBASE_ESR</code>: 40GBASE-ESR</p> </li>   <li> <p> <code>OPTIC_40GBASE_IR4_LR4L</code>: 40GBASE-IR (LR4L)</p> </li>   <li> <p> <code>OPTIC_40GBASE_LR4</code>: 40GBASE-LR4</p> </li>   <li> <p> <code>OPTIC_100GBASE_SR4</code>: 100GBASE-SR4</p> </li>   <li> <p> <code>OPTIC_100GBASE_CWDM4</code>: 100GBASE-CWDM4</p> </li>   <li> <p> <code>OPTIC_100GBASE_LR4</code>: 100GBASE-LR4</p> </li>   <li> <p> <code>OPTIC_100G_PSM4_MSA</code>: 100G PSM4 MSA</p> </li>   <li> <p> <code>OPTIC_1000BASE_LX</code>: 1000Base-LX</p> </li>   <li> <p> <code>OPTIC_1000BASE_SX</code> : 1000Base-SX</p> </li>  </ul>
    ///   - [`maximum_supported_weight_lbs(MaximumSupportedWeightLbs)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::maximum_supported_weight_lbs) / [`set_maximum_supported_weight_lbs(Option<MaximumSupportedWeightLbs>)`](crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::set_maximum_supported_weight_lbs): <p>The maximum rack weight that this site can support. <code>NO_LIMIT</code> is over 2000lbs. </p>
                            /// - On success, responds with [`UpdateSiteRackPhysicalPropertiesOutput`](crate::output::UpdateSiteRackPhysicalPropertiesOutput) with field(s):
    ///   - [`site(Option<Site>)`](crate::output::UpdateSiteRackPhysicalPropertiesOutput::site): <p>Information about a site.</p>
                            /// - On failure, responds with [`SdkError<UpdateSiteRackPhysicalPropertiesError>`](crate::error::UpdateSiteRackPhysicalPropertiesError)
    pub fn update_site_rack_physical_properties(&self) -> crate::client::fluent_builders::UpdateSiteRackPhysicalProperties {
                                crate::client::fluent_builders::UpdateSiteRackPhysicalProperties::new(self.handle.clone())
                            }
}

