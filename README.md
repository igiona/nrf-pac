# nrf-pac

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for Nordic Semiconductor nRF microcontrollers.

This crate has been automatically generated from the SVD files in [Nordic nrfx](https://github.com/NordicSemiconductor/nrfx/blob/master/mdk/), using [chiptool](https://github.com/embassy-rs/chiptool/). Fixes are added to the SVD file (via YAML file) to make the crate more amenable to writing HALs with, such as converting sets of identical registers/fields to arrays, merging identical registers and enums, etc.

This crate is used for the [`embassy-nrf`](github.com/embassy-rs/embassy/) Rust Hardware Abstraction Layer (HAL) for Nordic Semiconductor nRF microcontrollers.

## Supported chips

- **nrf52833**: [Datasheet](https://docs-be.nordicsemi.com/bundle/ps_nrf52833/attach/nRF52833_PS_v1.7.pdf?_LANG=enus)
- **nrf52840**: [Datasheet](https://docs-be.nordicsemi.com/bundle/ps_nrf52840/attach/nRF52840_PS_v1.10.pdf?_LANG=enus)
- **nrf5340** (both `application` and `network` cores): [Datasheet](https://docs-be.nordicsemi.com/bundle/ps_nrf5340/attach/nRF5340_PS_v1.5.pdf?_LANG=enus)

### Add support for a new chip

The following steps are required in order to add support for a new chip with name `<name>`:

- add the `<chip-identifier>.svd` and `<chip-identifier>.yaml` files into the `svd` folder
- add the `device-<chip-identifier>.x` file into the root folder
- append `<name>` to the `identifiers` associative array in `update.sh`, and set the value to `<chip-identifier>`
- append `<name>` to the `targets` associative array in `update.sh`, and set the value to appropriate CPU target value
- update this README file with the newly supported chip information
- run `update.sh`
- create PR with the updated files

## License

The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file, which is licensed by Nordic Semiconductor ASA under a BSD-3-Clause license.