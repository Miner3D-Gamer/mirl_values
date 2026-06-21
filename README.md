# TODO: This doesn't follow the mirl formatting standart!

### Mirl Values

> Doc written for version 0.0.0-alpha

#### Miva, a library to unify all (most) `Value` enums

> Sub-crate: `mirl_values_core` - Contains raw data types

<details>
<summary>Flags</summary>

### Default:

**Core**

- `std` (Default)
- `c_compatible`
- `all`

**Codec**

- `all_codecs`
- `serde`
- `bitcode`
- `wincode` (bitcode recommended)
- `zerocopy`
- `compactly`

**Enum**

- `all_enum_extensions`
- `strum`
- `enum_ext`

</details>

### Entry Points
> `prelude` provided

The `Value` struct sitting surface level

### Purpose

Context: Almost all parser libs have their own `Value` type as almost every format supports unique features.

This lib tries to support "all" of them, official support has been given for json, toml, and css (derivative formats like json5 are not mentioned).

This crate supports Wrappers - Wrap the `Value` enum in a struct to store additional data (Works with `Vec` and `Map`)

Supported types:

Simple:

- None
- Bool
- Number
- String
- Time (MM:HH)
- DateTime (Full datetime)
- Literal (String without quotes)
- Angle
- Length
- Color

Containers:

- Vec
- Map (Object/Dictionary)

### Disclaimer

This lib only contains data structures and conversion functions.
It is intended to be used by other libs.
For actually parsing objects, use [Mirl Codec Info](https://github.com/Miner3D-Gamer/mirl_codec_info).

### Origin
I wanted to add a css parser to `mirl_codec_info` but creating a new Value enum for every codec seemed redundant while simply extending the current one left the modularizing part of my brain unsatisfied; "What if you need Value enum for another format in the future? Are you just gonna duplicate all your existing code?".
