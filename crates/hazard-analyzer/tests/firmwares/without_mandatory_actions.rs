fn firmware() {

    let device = MethodsDevice::new()
        .first_action(DeviceAction::with_hazard(first_action_config, first_action, Hazard::PowerOutage))?
        .add_action(DeviceAction::no_hazards(optional_action_config, optional_action))?
        .state(device_state)
        .build()?;

}