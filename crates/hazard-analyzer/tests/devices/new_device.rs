// Imports and const definitions...

// Mandatory actions hazards.
const FIRST_ACTION: Hazard = Hazard::FireHazard;
const SECOND_ACTION: &[Hazard] = &[Hazard::FireHazard, Hazard::ElectricEnergyConsumption];

// Allowed hazards.
const ALLOWED_HAZARDS: &[Hazard] = &[Hazard::FireHazard, Hazard::ElectricEnergyConsumption];

pub struct NewDevice<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Main server route for device routes.
    main_route: &'static str,
    // Device state.
    state: Option<S>,
    // Device.
    device: Device<S>,
    // Allowed device hazards.
    allowed_hazards: &'static [Hazard],
}

impl<S> Device<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Creates a new [`NewDevice`] instance.
    pub fn new<H, T, H1, T1>(
        first_action: DeviceAction<H, T>,
        second_action: DeviceAction<H1, T1>,
        third_action: DeviceAction<H1, T1>,
    ) -> Result<Self>
    where
        H: Handler<T, ()>,
        T: 'static,
        H1: Handler<T1, ()>,
        T1: 'static,
    {
        // Raise an error whether turn first_action does not contain a
        // fire hazard.
        if first_action.miss_hazard(FIRST_ACTION) {
            return Err(Error::new(
                ErrorKind::NewDevice,
                "No fire hazards for the `first_action` route",
            ));
        }

        // Raise an error whether turn second_action does not contain
        // fire or electric energy consumption hazards.
        if second_action.miss_hazards(SECOND_ACTION) {
            return Err(Error::new(
                ErrorKind::NewDevice,
                "No fire or electric energy consumption hazards for the `second_action` route",
            ));
        }

        // Create a new device.
        let device = Device::new(DeviceKind::NewDevice)
            .add_action(first_action)
            .add_action(second_action)
            .add_action(third_action);

        Ok(Self {
            main_route: NEW_DEVICE_MAIN_ROUTE,
            device,
            state: None,
            allowed_hazards: ALLOWED_HAZARDS,
        })
    }

    /// Main route setting...

    /// Adds an additional action for a [`NewDevice`].
    pub fn add_action<H, T>(mut self, new_device_action: DeviceAction<H, T>) -> Result<Self>
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        // Return an error if action hazards are not a subset of allowed hazards.
        for hazard in new_device_action.hazards.iter() {
            if !self.allowed_hazards.contains(hazard) {
                return Err(Error::new(
                    ErrorKind::NewDevice,
                    format!("{hazard} hazard is not allowed for new device"),
                ));
            }
        }

        self.device = self.device.add_action(new_device_action);

        Ok(self)
    }

    /// Device state setting...

    /// Build...
}
