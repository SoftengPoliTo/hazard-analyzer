fn firmware_first_device() {

    let device = NewDevice::new(
        DeviceAction::no_hazards(first_action_config, first_action),
        DeviceAction::no_hazards(second_action_config, second_action),
        DeviceAction::no_hazards(third_action_config, third_action)
    )?
    .add_action(DeviceAction::no_hazards(optional_action_config, optional_action))?
    .state(device_state)
    .build();

}

fn firmware_second_device() {

    let device = MethodsDevice::new()
        .first_action(DeviceAction::no_hazards(first_action_config, first_action))?
        .second_action(DeviceAction::no_hazards(second_action_config, second_action))?
        .third_action(DeviceAction::no_hazards(third_action_config, third_action))
        .add_action(DeviceAction::no_hazards(optional_action_config, optional_action))?
        .state(device_state)
        .build()?;

}