// Imports and const definitions...

// Mandatory actions hazards.
const FIRST_ACTION: Hazard = Hazard::PowerOutage;
const SECOND_ACTION: &[Hazard] = &[Hazard::FireHazard, Hazard::SpoiledFood];

// Allowed hazards.
const ALLOWED_HAZARDS: &[Hazard] = &[Hazard::FireHazard, Hazard::SpoiledFood, Hazard::PowerOutage];

// Mandatory device actions.
#[derive(Debug, PartialEq, Eq, Hash)]
enum Actions {
    FirstAction,
    SecondAction,
    ThirdAction,
}

pub struct MethodsDevice<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Main server route for device routes.
    main_route: &'static str,
    // Device state.
    state: Option<S>,
    // Device.
    device: Device<S>,
    // Mandatory device actions.
    mandatory_actions: FnvIndexSet<Actions, MAXIMUM_ELEMENTS>,
    // Allowed device hazards.
    allowed_hazards: &'static [Hazard],
}

impl<S> MethodsDevice<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Creates a new [`MethodsDevice`] instance.
    pub fn new() -> Self {
        // Create a new device.
        let device = Device::new(DeviceKind::MethodsDevice);

        // Define mandatory actions.
        let mut mandatory_actions = FnvIndexSet::new();
        let _ = mandatory_actions.insert(Actions::IncreaseTemperature);
        let _ = mandatory_actions.insert(Actions::DecreaseTemperature);

        Self {
            main_route: METHODS_DEVICE_MAIN_ROUTE,
            device,
            state: None,
            mandatory_actions,
            allowed_hazards: ALLOWED_HAZARDS,
        }
    }

    /// Main route setting...

    /// Adds first action for a [`MethodsDevice`].
    pub fn first_action<H, T>(
        mut self,
        first_action: DeviceAction<H, T>,
    ) -> Result<Self>
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        // Raise an error whether first_action does not contain
        // power outage hazard.
        if first_action.miss_hazard(FIRST_ACTION) {
            return Err(Error::new(
                ErrorKind::MethodsDevice,
                "No power outage hazard for the `first_action` route",
            ));
        }

        self.device = self.device.add_action(first_action);

        // Remove first_action action from the list of actions to set.
        self.mandatory_actions.remove(&Actions::FirstAction);

        Ok(self)
    }

    /// Adds second action for a [`MethodsDevice`].
    pub fn second_action<H, T>(
        mut self,
        second_action: DeviceAction<H, T>,
    ) -> Result<Self>
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        // Raise an error whether second_action does not contain
        // fire or spoiled food hazards.
        if second_action.miss_hazards(SECOND_ACTION) {
            return Err(Error::new(
                ErrorKind::MethodsDevice,
                "No fire or spoiled food hazards for the `second_action` route",
            ));
        }

        self.device = self.device.add_action(second_action);

        // Remove second_action action from the list of actions to set.
        self.mandatory_actions.remove(&Actions::SecondAction);

        Ok(self)
    }

    /// Adds third action for a [`MethodsDevice`].
    pub fn third_action<H, T>(
        mut self,
        third_action: DeviceAction<H, T>,
    ) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.device = self.device.add_action(third_action);

        // Remove third_action action from the list of actions to set.
        self.mandatory_actions.remove(&Actions::ThirdAction);

        self
    }

    /// Adds an additional action for a [`MethodsDevice`].
    pub fn add_action<H, T>(mut self, methods_device_action: DeviceAction<H, T>) -> Result<Self>
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        // Return an error if action hazards are not a subset of allowed hazards.
        for hazard in methods_device_action.hazards.iter() {
            if !self.allowed_hazards.contains(hazard) {
                return Err(Error::new(
                    ErrorKind::MethodsDevice,
                    format!("{hazard} hazard is not allowed for methods device"),
                ));
            }
        }

        self.device = self.device.add_action(methods_device_action);

        Ok(self)
    }

    /// Device state setting...

    /// Builds a new [`Device`].
    pub fn build(self) -> Result<Device<S>> {
        // Return an error if not all mandatory actions are set.
        if !self.mandatory_actions.is_empty() {
            return Err(Error::new(
                ErrorKind::MethodsDevice,
                format!(
                    "The following mandatory actions are not set: {:?}",
                    self.mandatory_actions
                ),
            ));
        };

        let mut device = self.device.main_route(self.main_route).finalize();
        device.state = self.state;

        Ok(device)
    }
}
