# Hazard Analyzer

`hazard-analyer` analyzes the source code of a smart home firmware implemented using the `Ascot` interface to certify whether it is `Ascot` compliant or not. Specifically, for each instance of a device within the firmware it checks whether it meets certain conditions.

An `Ascot` device is characterized by a set of actions that must necessarily be defined in order to create an instance of it, consider for example the actions of turning a light on and off. 
These **mandatory actions** are then validated within the `Ascot` framework to ensure that the developer has assigned them all **mandatory hazards**. Referring again to the light example, the turn light on action must have a fire hazard attached to it.

Each device also has a set of **allowed hazards**, which is a list of all the hazards that can be associated with it.
In fact, a developer can add some actions in addition to the mandatory ones, with the condition that only the hazards allowed for that device are assigned to them.
This last condition must also be fulfilled when creating a mandatory action,  for the hazards that are assigned alongside the required ones.

To sum up, for each device:
- All mandatory actions must be defined.
- Each mandatory action must have all the hazards that are mandatory for it.
- The mandatory actions or any additional actions must not be associated with hazards that are not in the list of hazards allowed for the device.

## Workflow

The `hazard-analyzer` workflow involves an initial analysis of all devices provided by `Ascot` and defined in [ascot-firmware/ascot-axum/src/devices/](https://github.com/SoftengPoliTo/ascot-firmware/tree/master/ascot-axum/src/devices) to extract information about mandatory actions, hazards, and allowed hazards.

Then, given a firmware, all instances of `Ascot` devices in the various files are searched and checked to see if they satisfy the previously extracted conditions.

Both steps are based on a source code analysis performed with a fork, contained in the project root, of the [rust-code-analysis](https://github.com/mozilla/rust-code-analysis) library.

The output of the `hazard-analyzer` is a manifest file in `JSON` format containing a list of all the files within which devices are instantiated, and for each device, it shows its location within the file along with all the information about the actions and hazards associated with it, including lists of missing mandatory actions or hazards and those of not allowed hazards.

The tool also prints a similar output to that of the manifest in the terminal, in which the errors associated with the definition of the various devices are highlighted in red.

## Building

Use this command to build the tool:

```console
cargo build 
```

## Testing

Testing has been performed via snapshots, using [insta](https://insta.rs). Use the following command to launch the tests:

``` console
cargo insta test
```

To review the snapshots, use:

``` console
cargo insta review
```

The next command combines the previous two operations:

``` console
cargo insta test --review
```
