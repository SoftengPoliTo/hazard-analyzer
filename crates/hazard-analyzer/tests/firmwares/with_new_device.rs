fn firmware() {

    let device = NewDevice::new(
        DeviceAction::with_hazard(first_action_config, first_action, Hazard::FireHazard),
        DeviceAction::with_hazards(second_action_config, second_action, &[Hazard::ElectricEnergyConsumption, Hazard::FireHazard]),
        DeviceAction::no_hazards(third_action_config, third_action)
    )?
    .add_action(DeviceAction::no_hazards(optional_action_config, optional_action))?
    .state(device_state)
    .build();

}
