# pt-rtd

[![Latest Version](https://img.shields.io/crates/v/pt-rtd.svg)][`pt-rtd`]
[![Documentation](https://docs.rs/pt-rtd/badge.svg)][`pt-rtd`/docs]
[![License](https://img.shields.io/crates/l/pt-rtd.svg)][`pt-rtd`/license]
[![Dependency Status](https://deps.rs/repo/github/thecodechemist99/pt-rtd/status.svg)][`pt-rtd`/dep_status]

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pt-rtd = "0.1"
```

## Examples

```rust,ignore
use pt_rtd::{
    self as rtd,
    RTDType,
};

fn main() -> ! {
    let resistance: f32 = 100.0;
    let temperature: f32 = 0.0;

    // Convert resistance to temperature
    let result = rtd::calc_t(resistance, RTDType::PT100);
    let t = match result {
        Ok(temp) => temp,
        Err(e) => // handle error
    }

    // Convert temperature to resistance
    let result = rtd::calc_r(temperature, RTDType::PT100);
    let r = match result {
        Ok(res) => res,
        Err(e) => // handle error
    }
}
```

For relative mesurements, the library can also convert the ADC reading to a resistance value:

```rust,ignore
use pt_rtd::{
    self as rtd,
    ADCRes,
};

fn main() -> ! {
    let adc_value: u32 = 1000;
    let adc_resolution = ADCRes::B24;
    let ref_resistance = 5600;
    let pga_gain = 16;

    let result = rtd::conv_d_val_to_r(adc_value, ref_resistance, adc_resolution, pga_gain);
    let r = match result {
        Ok(res) => res,
        Err(e) => // handle error
    }
}
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
