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
├── tmp
└── var
    ├── counters.pc
    └── log
        ├── debug.log
        └── fault.log
```

## /etc

This is where all configuration is stored.

### calib.pc

Calibration data, like IMU/current sensor/etc offsets, hall tables, etc etc.

### sys.toml

System configuration. Stuff like current limits, IMU rotation, etc.

### profiles.d/

All ride profiles. Built-in profiles are not stored here; rather they are hardcoded into
the firmware, here be only user-created profiles, for example when a profile is edited.

## /tmp

Any temporary files the system might create.

## /var

### log/

#### debug.log

This is the defmt log for debugging purposes. Contains all log levels except for `trace`.

#### fault.log

If there are any faults recorded, this log will have them. If its not empty, the
"check engine" light will pop up (or any other way of signifying that there was an error).
