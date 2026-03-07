# Firmware updates

This document is a ChatGPT draft. It is meant to capture the current intended design, not a
final specification.

This page describes the intended UniLANCE firmware update flow, especially for systems with
multiple MCUs such as a supervisor MCU and a control MCU.

The intended bootloader framework is `embassy-boot`.

## Goals

- supervisor is the system update orchestrator
- each MCU updates its own firmware image
- official firmware updates are signed and encrypted
- per-MCU root keys live in OTP
- entitlement is shared at the model-batch level
- control can receive updates from supervisor over the inter-MCU link
- custom firmware unlock remains possible and irreversible

## Trust model

The update trust model is:

- official updates are vendor-signed
- official update payloads are model-batch-encrypted
- each MCU has its own root key in OTP
- each MCU combines its own root key with the shared entitlement to derive its decryption path

This means:

- the entitlement is not treated as secret
- the root key is the critical secret
- supervisor and control must not share the same root key

## Node responsibilities

### Supervisor

The supervisor is responsible for:

- receiving update packages from the app, host, or storage
- staging update packages in persistent storage
- verifying the signed update manifest
- deciding which node images exist in the package
- orchestrating distribution of those images to other MCUs
- applying its own firmware update through its own boot path

The supervisor is the update coordinator, not the sole trust anchor for every other MCU.

### Control

The control MCU is responsible for:

- receiving its image from supervisor
- deriving its decryption capability from its own root key plus shared entitlement
- decrypting incoming encrypted control firmware packets on the fly
- writing the result into its own inactive firmware slot
- verifying the final image hash before marking the update as ready
- letting its own bootloader perform the final boot-time swap and validation flow

### Bootloaders

Each independently booting MCU should own its own `embassy-boot` state and image slots.

That means:

- supervisor bootloader manages supervisor images
- control bootloader manages control images
- no MCU writes directly over another MCU's active image

## Package structure

An update package should contain at least:

- signed manifest
- one or more encrypted firmware payloads
- target metadata for each payload
- image hashes
- version information
- optional staging entitlement blob for future updates

One package may contain multiple images, for example:

- supervisor firmware
- control firmware
- future BMS or display firmware

The manifest should describe each image by logical target, not just filename.

Examples:

- `supervisor`
- `control`
- `bms1`
- `display`

## Why only hash verification on control

The current intended design is:

- supervisor verifies the signed manifest
- control verifies the final image hash

In other words, control does not need to verify the vendor signature on the manifest again.
Instead, control trusts supervisor to authenticate the package and trusts itself to confirm
that the exact expected image was received and written correctly.

That is a deliberate simplification for this architecture.

This means the control-side trust model is:

- supervisor authenticates the update package
- control enforces transfer correctness and boot correctness

## Single-MCU update flow

For a node updating itself from locally staged storage, the intended flow is:

1. read the manifest
2. verify the vendor signature on the manifest
3. verify target compatibility and version policy
4. combine local root key with shared entitlement
5. decrypt the encrypted firmware payload
6. verify the resulting image hash
7. write the image to the inactive `embassy-boot` slot
8. request swap/reboot
9. bootloader performs the swap flow

## Multi-MCU update flow

For a supervisor distributing an update to control, the intended flow is:

1. supervisor receives and stages the update package
2. supervisor verifies the vendor signature on the manifest
3. supervisor verifies package compatibility and policy
4. supervisor determines that the package contains a control image
5. supervisor instructs control to enter update-receive mode
6. control prepares its inactive `embassy-boot` slot and update state
7. control loads or derives its decryption state in RAM from:
   - its local root key in OTP
   - the shared entitlement blob
8. supervisor streams encrypted control-image packets over the inter-MCU link
9. control decrypts packets on the fly and writes plaintext into its inactive slot
10. control computes and verifies the final image hash
11. control acknowledges successful receipt
12. supervisor records that control update staging succeeded
13. control reboots when policy allows
14. control bootloader performs the `embassy-boot` swap flow

## Why decrypt on control

The current preferred design is for control to decrypt its own image while receiving it.

That gives several benefits:

- control keeps ownership of its own root key usage
- large encrypted staging space on control is not required
- supervisor can remain the transport/orchestration node instead of the decryption authority
- each MCU remains more self-contained for future expansion

## Root keys and entitlement

The agreed design is:

- each MCU has its own root key in OTP
- entitlement is shared per model batch
- entitlement may be stored in the filesystem and is not considered secret
- the root key must never be shared between supervisor and control

This gives a useful split:

- leaking entitlement alone does not break security
- leaking one MCU's root key does not automatically leak the other MCU's root key
- update confidentiality is managed per model batch, not per individual wheel

## Entitlement rotation

An update may carry a staging entitlement blob.

If it does, the rule is:

- the current update is always decrypted using the current root-key-plus-entitlement chain
- the staging entitlement only becomes active for subsequent updates
- entitlement state follows the same active/staging model as firmware slots

Recommended flow:

1. authenticate and process the current update with the current trust chain
2. install or stage the current firmware image successfully
3. store the new entitlement as staging, not active
4. keep the current active entitlement unchanged
5. when the new firmware is committed, promote the staging entitlement alongside it

Do not overwrite the active entitlement in place.

The reason for this is rollback safety. If the newly staged firmware fails and the system
returns to the old image, the old active entitlement must still match that old image chain.
Ignoring general filesystem state for now, this gives the system a clean way to restore the
original firmware-entitlement state pair.

Recommended draft naming for writable entitlement state:

- `/var/lib/unilance/security/entitlement-active.pc`
- `/var/lib/unilance/security/entitlement-staging.pc`

These paths are not special because of the filenames themselves, but because they make the
pairing model explicit and easy to reason about.

These entitlement files are intentionally kept in the filesystem. The point is to avoid using
internal flash for this kind of frequently managed update state and reduce unnecessary wear on
the MCU's own flash.

## Update transport between nodes

The inter-MCU transfer protocol is still work in progress, but the intended behavior is:

- supervisor starts an update session for a specific logical target
- control acknowledges readiness
- supervisor sends encrypted packets in order
- control acknowledges or rejects packets
- control reports final image-hash result
- supervisor commits overall session state

The existing file-transfer messages in CORElink are a reasonable starting point, but firmware
update should be treated as a distinct state machine, not just a generic file copy.

At minimum, update transfer needs:

- session start
- target selection
- packet sequencing
- retransmission support
- transfer completion
- final hash confirmation
- abort/reset behavior

## `embassy-boot` usage

`embassy-boot` should be used per MCU in the standard inactive-slot style.

That means:

- the running image is never overwritten in place
- the new image is written to the inactive slot
- swap happens through bootloader state on reboot
- failure handling and rollback should follow `embassy-boot` semantics

For UniLANCE, this implies each independently booting MCU should have:

- bootloader region
- active firmware slot
- inactive firmware slot
- boot state / swap state region

## Failure handling

A good update design must tolerate interruption.

Supervisor-side failures:

- interrupted download
- bad signature
- incompatible target
- failed transfer to control

Control-side failures:

- packet loss
- decrypt failure
- flash write failure
- final hash mismatch
- boot validation failure after reboot

Expected behavior:

- never touch the active image while staging a new one
- do not replace entitlement early
- keep update state resumable or restartable where practical
- fall back to the old image if the new one is not committed successfully

## Unlock interaction

Developer unlock must permanently disable the official encrypted-update path.

That means unlocking an MCU should at minimum:

- erase or revoke its ability to use the official entitlement path
- set an irreversible lifecycle bit in OTP
- optionally lower RDP depending on policy

After unlock:

- that MCU may accept custom firmware according to policy
- that MCU must no longer be able to use official encrypted updates

This is per MCU. Unlocking control does not automatically imply unlocking supervisor, and vice
versa, unless system policy explicitly says so.

## Suggested lifecycle

The intended practical lifecycle is:

- `shipping_locked`
- `update_staged`
- `swap_pending`
- `running_new_image`
- `developer_unlocked`

Exact naming can change later, but the important part is that unlock remains one-way.

## Summary

The intended UniLANCE update model is:

- supervisor orchestrates updates for the whole system
- official packages are signed and encrypted
- supervisor verifies the signed manifest
- each MCU has its own OTP root key
- entitlement is shared per model batch and is not treated as secret
- control decrypts its own incoming firmware stream using its own root key plus entitlement
- control verifies final image hash, then `embassy-boot` handles the swap
- staging entitlement blobs, if present, apply only to future updates and are promoted only
  together with the corresponding committed firmware state
- developer unlock permanently disables the official encrypted update path on the unlocked MCU

For the related trust and storage drafts, see:

- `docs/drafts/SECURE_BOOT_AND_KEYS.md`
- `docs/drafts/FILESYSTEM.md`
