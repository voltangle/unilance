# Filesystem architecture

The filesystem design tries to mimic UNIX as much as possible, because I like it that way.
It will generally look like this:

```
├── etc
│   ├── calib.pc
│   ├── profiles.d
│   │   ├── offroad.toml
│   │   ├── sport.toml
│   │   └── street.toml
│   └── sys.toml
├── sys
│   ├── defaults
│   │   ├── profiles.d
│   │   │   ├── offroad.toml
│   │   │   ├── sport.toml
│   │   │   └── street.toml
│   │   └── sys.toml
│   ├── last_boot_reason
│   └── version
├── tmp
└── var
    ├── lib
    │   └── vehicle
    │       ├── ble.conf
    │       └── counters.pc
    ├── log
    │   ├── debug.log
    │   └── fault.log
    └── spool
```

## /etc

This is where all configuration is stored.

### calib.pc

Calibration data, like IMU/current sensor/etc offsets, hall tables, etc etc.

### profiles.d

All ride profiles. Built-in profiles are not stored here; rather they are hardcoded into
the firmware, here be only user-created profiles, for example when a profile is edited.
