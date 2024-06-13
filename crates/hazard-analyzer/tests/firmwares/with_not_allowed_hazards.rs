fn firmware_first_device() {

    let device = NewDevice::new(
        DeviceAction::with_hazard(first_action_config, first_action, Hazard::FireHazard),
        DeviceAction::with_hazards(second_action_config, second_action, &[Hazard::ElectricEnergyConsumption, Hazard::FireHazard, Hazard::PowerSurge]),
        DeviceAction::no_hazards(third_action_config, third_action)
    )?
    .add_action(DeviceAction::with_hazards(optional_action_config, optional_action, &[Hazard::PowerSurge, Hazard::PowerOutage]))?
    .state(device_state)
    .build();

}

fn firmware_second_device() {

    let device = MethodsDevice::new()
        .first_action(DeviceAction::with_hazard(first_action_config, first_action, Hazard::PowerOutage))?
        .second_action(DeviceAction::with_hazards(second_action_config, second_action, &[Hazard::FireHazard, Hazard::SpoiledFood, Hazard::ElectricEnergyConsumption]))?
        .third_action(DeviceAction::no_hazards(third_action_config, third_action))
        .add_action(DeviceAction::with_hazard(optional_action_config, optional_action, Hazard::ElectricEnergyConsumption))?
        .state(device_state)
        .build()?;

}