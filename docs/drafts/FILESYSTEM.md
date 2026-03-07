# Filesystem architecture

This document is a ChatGPT draft. It is meant to capture the current intended design, not a
final specification.

UniLANCE uses a Linux-like filesystem split into two partitions:

- `system`: immutable, replaced as a whole during system updates
- `data`: writable, preserved across updates except when migrations or an explicit reset say
  otherwise

The intended model is similar to immutable Linux systems such as Fedora Silverblue, or modern
macOS system/data separation:

- the base system image is read-only from firmware code
- persistent mutable state lives elsewhere
- local overrides and user data survive normal system replacement

## Design rules

- keep the logical directory layout as UNIX-like as possible
- keep shipped defaults in the immutable `system` partition
- keep persistent writable state in the `data` partition
- store normal config/state as compact `.pc` files
- use `.toml` only for sparse developer overrides of hidden or unsafe options
- never use TOML as the primary storage format for ordinary settings

## High-level layout

The runtime should look roughly like this:

```text
/
├── etc
│   └── override.d
│       ├── balance.toml
│       ├── imu.toml
│       ├── motor.toml
│       └── unsafe.toml
├── tmp
├── usr
│   ├── lib
│   │   └── unilance
│   │       ├── build.pc
│   │       ├── control-defaults.pc
│   │       ├── profiles-defaults.pc
│   │       ├── schema.pc
│   │       ├── supervisor-defaults.pc
│   │       └── system.pc
│   └── share
│       └── unilance
│           ├── assets
│           ├── docs
│           └── profiles
└── var
    ├── lib
    │   └── unilance
    │       ├── calib.pc
    │       ├── counters.pc
    │       ├── machine.pc
    │       ├── migrations.pc
    │       ├── profiles.pc
    │       ├── selected-profile.pc
    │       ├── settings.pc
    │       └── state.pc
    ├── log
    │   ├── boot.log
    │   ├── debug.log
    │   └── fault.log
    └── spool
        └── update
```

## Partition backing

- `system` backs `/usr/...`
- `data` backs `/etc/...` and `/var/...`
- `/run` and `/tmp` are volatile

In other words:

- `/usr` is firmware-owned and replaced on system update
- `/etc` is local administrator or developer override space
- `/var` is persistent mutable state

## Why `/etc` is small here

On UniLANCE, `/etc` is not the main place where normal settings live.

Normal config and state are stored in `.pc` files under `/var/lib/unilance`, because:

- they are compact
- they are typed
- they are easier to validate
- they are a better fit for app-driven editing on embedded storage

TOML files in `/etc/override.d` exist only to create friction around hidden, expert-only, or
unsafe settings.

If a setting can be safely edited by the consumer UI, it should generally not require a TOML
file at all.

## File-by-file meaning

### `/etc`

`/etc/override.d/`
- developer-only sparse overrides
- intended for hidden, advanced, or unsafe knobs
- missing file or missing key means no override

`/etc/override.d/motor.toml`
- advanced motor-control overrides

`/etc/override.d/balance.toml`
- advanced balancing algorithm overrides

`/etc/override.d/imu.toml`
- advanced IMU and estimation overrides

`/etc/override.d/unsafe.toml`
- intentionally high-friction overrides for options that can reduce safety margins or damage
  hardware

### `/usr/lib/unilance`

These are immutable machine-readable defaults shipped in the system image.

`/usr/lib/unilance/system.pc`
- top-level immutable system defaults for the target or model

`/usr/lib/unilance/control-defaults.pc`
- immutable defaults for the control subsystem

`/usr/lib/unilance/supervisor-defaults.pc`
- immutable defaults for the supervisor subsystem

`/usr/lib/unilance/profiles-defaults.pc`
- immutable stock ride profiles shipped with the system image

`/usr/lib/unilance/schema.pc`
- schema/version information used for migration decisions

`/usr/lib/unilance/build.pc`
- build metadata for the installed system image, such as version, target, or compatibility

### `/usr/share/unilance`

These are immutable human-facing assets.

`/usr/share/unilance/assets/`
- UI and static product assets

`/usr/share/unilance/docs/`
- packaged on-device documentation or reference material, if desired

`/usr/share/unilance/profiles/`
- optional unpacked stock profile resources if they need to exist as individual files

### `/var/lib/unilance`

These are writable persistent machine-owned files.

`/var/lib/unilance/settings.pc`
- main writable settings blob used by the UI and app for ordinary settings

`/var/lib/unilance/calib.pc`
- calibration data such as IMU offsets, current-sensor offsets, hall data, and similar learned
  parameters

`/var/lib/unilance/counters.pc`
- odometer-like counters, maintenance counters, or other monotonic stats

`/var/lib/unilance/state.pc`
- last-known operational state that is worth restoring but is not exactly user settings

`/var/lib/unilance/profiles.pc`
- writable user-created or user-modified profiles

`/var/lib/unilance/selected-profile.pc`
- active profile selection

`/var/lib/unilance/machine.pc`
- local machine-specific writable metadata that does not belong in OTP, such as provisioning
  side data or non-critical local identity metadata

`/var/lib/unilance/migrations.pc`
- migration bookkeeping and applied-schema history

### `/var/log`

`/var/log/debug.log`
- persistent debug log for troubleshooting

`/var/log/fault.log`
- persistent fault/event log intended for service diagnostics and user-visible fault presence

`/var/log/boot.log`
- boot and early-startup log

### `/var/spool/update`

`/var/spool/update/`
- staging area for update payloads before verification and installation

### Volatile paths

`/tmp`
- temporary files safe to discard on reboot

## Config precedence

Runtime config should be constructed in this order:

1. immutable defaults from `/usr/lib/unilance/*.pc`
2. writable persistent state from `/var/lib/unilance/*.pc`
3. sparse developer overrides from `/etc/override.d/*.toml`

That means:

- `.pc` files are the primary source of truth for normal operation
- TOML only overrides specific keys explicitly present in override files

## Override semantics

TOML overrides should follow these rules:

- sparse only; never write full config snapshots there
- absent file means no override
- absent key means no override
- invalid keys or values must be rejected and logged clearly
- consumer-facing tools should not edit these files
- developer tools may edit these files behind an explicit expert mode

Overrides should still pass through the same validation rules as app-written settings.

## Profiles

Profiles should not primarily live as TOML files.

Recommended split:

- stock profiles in immutable `.pc` defaults under `/usr/lib/unilance/profiles-defaults.pc`
- user-created or modified profiles in `/var/lib/unilance/profiles.pc`
- active selection in `/var/lib/unilance/selected-profile.pc`

This makes profiles compact, app-friendly, and easy to preserve across updates.

## Update behavior

Normal system update behavior should be:

- replace `system` as a whole
- preserve `data`
- run migrations if the schema version changes

In practice this means:

- `/usr/...` is replaced by the new system image
- `/etc/...` and `/var/...` are kept unless migration or reset says otherwise

## Security-sensitive data

Firmware entitlement state used for official encrypted updates should live in a dedicated
sensitive location in writable storage, not mixed casually with normal user-facing config.

That storage should support both active and staging entitlement blobs, mirroring the same
active/inactive or active/staging idea used for firmware updates themselves.

Recommended layout:

- `/var/lib/unilance/security/entitlement-active.pc`
- `/var/lib/unilance/security/entitlement-staging.pc`

The intended meaning is:

- `entitlement-active.pc` is the currently trusted entitlement blob
- `entitlement-staging.pc` is the entitlement blob paired with a staging firmware update, if one
  exists

Promotion should happen together with the corresponding firmware commit, not earlier.

These entitlement files intentionally live in the filesystem rather than internal flash. One
of the filesystem's jobs is to absorb this kind of update-state churn and avoid unnecessary
wear on MCU internal flash.

That design is described in `docs/drafts/SECURE_BOOT_AND_KEYS.md` and
`docs/drafts/FIRMWARE_UPDATES.md`.

## Variants

Target variants should be represented as different immutable system defaults, not different
binaries.

That means a target variant is mainly a different `system` image, especially different
contents under `/usr/lib/unilance/*.pc`.

## Summary

The intended UniLANCE filesystem model is:

- immutable `system` partition for shipped defaults and assets
- writable `data` partition for state, logs, calibration, and user-managed content
- `.pc` as the primary format for normal config and state
- `.toml` used only for sparse expert overrides in `/etc/override.d`
- Linux-like logical layout with `/usr`, `/etc`, `/var`, `/run`, and `/tmp`
