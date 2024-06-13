fn firmware() {

    let device = MethodsDevice::new()
        .first_action(DeviceAction::with_hazard(first_action_config, first_action, Hazard::PowerOutage))?
        .second_action(DeviceAction::with_hazards(second_action_config, second_action, &[Hazard::FireHazard, Hazard::SpoiledFood]))?
        .third_action(DeviceAction::no_hazards(third_action_config, third_action))
        .add_action(DeviceAction::no_hazards(optional_action_config, optional_action))?
        .state(device_state)
        .build()?; 
    
}
