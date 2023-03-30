// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeregisterRobot`](crate::client::fluent_builders::DeregisterRobot) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet(impl Into<String>)`](crate::client::fluent_builders::DeregisterRobot::fleet) / [`set_fleet(Option<String>)`](crate::client::fluent_builders::DeregisterRobot::set_fleet): <p>The Amazon Resource Name (ARN) of the fleet.</p>
    ///   - [`robot(impl Into<String>)`](crate::client::fluent_builders::DeregisterRobot::robot) / [`set_robot(Option<String>)`](crate::client::fluent_builders::DeregisterRobot::set_robot): <p>The Amazon Resource Name (ARN) of the robot.</p>
                            /// - On success, responds with [`DeregisterRobotOutput`](crate::output::DeregisterRobotOutput) with field(s):
    ///   - [`fleet(Option<String>)`](crate::output::DeregisterRobotOutput::fleet): <p>The Amazon Resource Name (ARN) of the fleet.</p>
    ///   - [`robot(Option<String>)`](crate::output::DeregisterRobotOutput::robot): <p>The Amazon Resource Name (ARN) of the robot.</p>
                            /// - On failure, responds with [`SdkError<DeregisterRobotError>`](crate::error::DeregisterRobotError)
    #[deprecated(note = "Support for the AWS RoboMaker application deployment feature has ended. For additional information, see https://docs.aws.amazon.com/robomaker/latest/dg/fleets.html.")]
    pub fn deregister_robot(&self) -> crate::client::fluent_builders::DeregisterRobot {
                                crate::client::fluent_builders::DeregisterRobot::new(self.handle.clone())
                            }
}

