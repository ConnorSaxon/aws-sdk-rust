// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartBgpFailoverTest`](crate::client::fluent_builders::StartBgpFailoverTest) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`virtual_interface_id(impl Into<String>)`](crate::client::fluent_builders::StartBgpFailoverTest::virtual_interface_id) / [`set_virtual_interface_id(Option<String>)`](crate::client::fluent_builders::StartBgpFailoverTest::set_virtual_interface_id): <p>The ID of the virtual interface you want to test.</p>
    ///   - [`bgp_peers(Vec<String>)`](crate::client::fluent_builders::StartBgpFailoverTest::bgp_peers) / [`set_bgp_peers(Option<Vec<String>>)`](crate::client::fluent_builders::StartBgpFailoverTest::set_bgp_peers): <p>The BGP peers to place in the DOWN state.</p>
    ///   - [`test_duration_in_minutes(i32)`](crate::client::fluent_builders::StartBgpFailoverTest::test_duration_in_minutes) / [`set_test_duration_in_minutes(Option<i32>)`](crate::client::fluent_builders::StartBgpFailoverTest::set_test_duration_in_minutes): <p>The time in minutes that the virtual interface failover test will last.</p>  <p>Maximum value: 180 minutes (3 hours).</p>  <p>Default: 180 minutes (3 hours).</p>
                            /// - On success, responds with [`StartBgpFailoverTestOutput`](crate::output::StartBgpFailoverTestOutput) with field(s):
    ///   - [`virtual_interface_test(Option<VirtualInterfaceTestHistory>)`](crate::output::StartBgpFailoverTestOutput::virtual_interface_test): <p>Information about the virtual interface failover test.</p>
                            /// - On failure, responds with [`SdkError<StartBgpFailoverTestError>`](crate::error::StartBgpFailoverTestError)
    pub fn start_bgp_failover_test(&self) -> crate::client::fluent_builders::StartBgpFailoverTest {
                                crate::client::fluent_builders::StartBgpFailoverTest::new(self.handle.clone())
                            }
}

