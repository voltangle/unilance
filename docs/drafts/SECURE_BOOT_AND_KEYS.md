# Secure Boot and Firmware Keys

This document is a ChatGPT draft. It is meant to capture the current intended design, not a
final security specification.

This page describes a minimal secure-update design for UniLANCE on hardware like the
STM32F405, using:

- OTP memory for a tiny immutable trust anchor
- external NOR flash for the filesystem and update payloads
- signed and encrypted official firmware updates
- a one-way developer unlock flow

The goals are:

- only accept official firmware in locked mode
- keep official firmware blobs encrypted at rest to protect IP
- allow entitlement rotation on arbitrary releases
- allow a permanent owner-developer unlock that disables official encrypted updates

This is not meant to resist a nation-state lab. It is mainly an anti-extraction,
anti-repackaging, and anti-casual-cloning design.

## Terminology

- `root key`: immutable per-device secret stored in OTP
- `KEK`: key-encryption key derived from the root key
- `entitlement blob`: wrapped erasable secret stored in the filesystem
- `active entitlement`: the currently trusted entitlement blob
- `staging entitlement`: a replacement entitlement blob that is not active yet
- `manifest`: signed metadata describing an update payload

## Trust model in one sentence

Official updates are vendor-signed and model-batch-encrypted, while wheel identity and
lifecycle state remain per-device via OTP.

## Core idea

UniLANCE uses a two-part trust chain:

```text
OTP root key + filesystem entitlement blob = ability to decrypt official firmware
```

The root key alone is not enough.
The entitlement blob alone is not enough.
Both are required.

This is the main reason for the split:

- the root key is immutable and device-bound
- the entitlement blob is erasable and rotatable

In the current intended model, entitlement is managed per model batch rather than per wheel.
That means devices of the same model family may share the same official update entitlement
material conceptually, while still keeping per-device identity and lifecycle bits in OTP.

That gives the device a real unlock path. OTP cannot be erased, but official update
decryption can still be permanently disabled by destroying the entitlement blob and marking
the device as unlocked.

## What goes into OTP

The STM32F405 OTP region is small, so OTP should only contain compact immutable anchors.

Recommended contents:

- format/version marker
- compact serial number or compact serial fields
- hardware revision / target ID
- manufacturing flags
- lifecycle flags
- per-device root key
- per-device serial number or serial fields used as the primary device identity
- optional fingerprint of the device certificate or public key
- integrity field such as a CRC

Good manufacturing flags:

- factory test complete
- calibration complete
- provisioned

Good lifecycle flags:

- shipping locked
- developer unlocked
- RMA mode

Lifecycle must be forward-only. A device may go from locked to unlocked, but it must not be
able to regain trusted factory state.

## What goes into NOR or normal flash

Recommended contents:

- encrypted firmware payloads
- signed manifests
- active wrapped entitlement blob
- staging wrapped entitlement blob, if an update carries one
- full device certificate or cert chain, if needed
- normal filesystem data

The important rule is that the entitlement blob must never be stored in plaintext.

The filesystem itself does not need to be fully encrypted. Only sensitive blobs such as the
entitlement need wrapping.

Entitlements are intentionally stored in the filesystem rather than internal flash. One of the
main reasons for the filesystem design is to avoid unnecessary strain and wear on internal
flash.

Recommended entitlement storage naming:

- `/var/lib/unilance/security/entitlement-active.pc`
- `/var/lib/unilance/security/entitlement-staging.pc`

These names are only a draft convention, but the important part is the semantics:

- active entitlement is the blob used by the current committed system state
- staging entitlement is the blob paired with a staging firmware update and must not become
  active until that firmware state is committed

## Signing and encryption

Official firmware updates should be both:

- signed, for authenticity and anti-tamper
- encrypted, for confidentiality and IP protection

Both are needed.

Encryption alone does not stop malicious firmware replacement.
Signing alone does not stop firmware disclosure from update packages.

## Minimal locked update flow

In locked mode, the update process should be:

1. read the manifest
2. verify the vendor signature on the manifest
3. verify target compatibility and version policy
4. derive a KEK from the OTP root key
5. unwrap the active entitlement blob
6. decrypt the firmware payload using the current key chain
7. verify the decrypted firmware hash
8. install the firmware
9. if the update carries a new wrapped entitlement blob, store it as staging for future
   updates only

The update that carries a staging entitlement is still decrypted using the active trusted
chain. The staging entitlement only becomes active for subsequent updates.

This avoids circular dependency problems and keeps update trust progression simple.

## Entitlement rotation

Entitlement rotation is allowed on arbitrary releases.

An official update package may optionally contain a staging wrapped entitlement blob.
If present, the update process should:

1. decrypt and validate the current update using the current root key + entitlement chain
2. install the update successfully
3. atomically save the new entitlement as the staging entitlement
4. keep the current active entitlement unchanged for the current booted system state
5. use the staging entitlement only after the update has committed and become the new active
   system state

Important rule:

- do not overwrite the active entitlement in place

Entitlements should follow the same active/staging pattern as firmware slots. This allows safe
rollback and full restoration of the original firmware-entitlement state pair, ignoring normal
filesystem state for now.

The general update flow is described in `docs/drafts/FIRMWARE_UPDATES.md`, and the storage
layout is described in `docs/drafts/FILESYSTEM.md`.

## Why not store the direct update key in OTP

Because OTP is not erasable.

If OTP alone is sufficient to decrypt official firmware, then the device can never truly lose
that capability. That breaks the idea of a permanent owner unlock.

By making official firmware decryption depend on both:

- immutable OTP root key
- erasable entitlement blob

the decryption path can be permanently revoked.

## Developer unlock flow

The device should support a one-way owner-developer unlock flow.

The purpose of this flow is to allow:

- custom firmware installation
- optional lowering of RDP for debug access
- permanent revocation of official encrypted update decryption

Recommended flow:

1. authenticate the unlock request through the official developer tool
2. show strong warnings in the tool and, if possible, on-device
3. erase the entitlement blob
4. erase other sensitive trust material from flash and NOR
5. erase local credentials, pairing state, and update staging data
6. optionally erase the data partition
7. set irreversible OTP lifecycle flag `developer_unlocked`
8. optionally lower RDP to 0, knowing this will mass-erase internal flash

After this point, the device:

- may accept custom firmware according to bootloader policy
- may allow debugging if RDP was lowered
- must not be able to decrypt official encrypted updates anymore

This state should be permanent.

## What should be wiped during unlock

At minimum:

- active entitlement blob
- any staging entitlement blobs
- device trust metadata used only for official locked mode
- pairing secrets and tokens
- update staging area

Usually also desirable:

- filesystem data partition
- logs
- calibration and tuning data

Not wipeable in this architecture:

- OTP serial number
- OTP root key
- OTP lifecycle bits already programmed

That is acceptable. The important property is that the root key becomes useless for official
updates once the entitlement is gone and the device is marked unlocked.

## Device identity

If the device needs a certificate for app or backend identity:

- store the full certificate in NOR or normal flash
- store only its fingerprint or public-key hash in OTP

That gives a compact immutable identity anchor without wasting OTP on full certificate data.

## Threat model

This design mainly protects against:

- offline extraction of plaintext firmware from update payloads
- unauthorized replacement of official firmware when locked
- continued use of official encrypted updates after an intentional unlock

This design does not fully protect against:

- invasive extraction of secrets from a determined lab attacker
- total compromise of a live device with all protections bypassed
- perfect secrecy once a per-device root key is recovered
- batch-wide confidentiality loss if one model-batch decryption path is fully recovered

If the root key is extracted from one device and the attacker also has the wrapped
entitlement blob, that device's official firmware decryption path is compromised. With
model-batch entitlement, that may also compromise confidentiality for official updates of that
model batch. This is an accepted tradeoff for simpler manufacturing and server-side key
management.

## Minimal manufacturing flow

One possible provisioning flow:

1. generate or assign a per-device root key
2. burn OTP structure: serial, target ID, flags, root key
3. derive a KEK from the root key
4. create or fetch the current model-batch entitlement blob and wrap it with that KEK
5. write the wrapped entitlement blob to the active entitlement location
6. write device certificate and metadata if needed
7. enable RDP for shipping firmware
8. mark the lifecycle as `shipping locked`

## Minimal locked boot/update flow

1. read OTP identity structure
2. verify lifecycle state is locked
3. read manifest and payload metadata
4. verify signature
5. derive KEK from OTP root key
6. unwrap the active entitlement blob
7. decrypt firmware payload
8. verify final firmware hash
9. install and boot
10. if present, keep the staging entitlement for the newly staged firmware and only promote it
    as active when the update is committed

## Minimal unlocked boot flow

When the device is marked `developer_unlocked`:

- the bootloader must not attempt official firmware decryption
- the bootloader may allow unsigned or user-signed images, depending on policy
- the old official encrypted-update path must remain disabled permanently

## Notes for filesystem integration

This design fits the immutable `system` + writable `data` model well.

- `system` may contain shipped public metadata and update logic
- `data` may contain the entitlement blob, update staging data, and normal persistent state

The entitlement blob should live in a dedicated sensitive path or region, not mixed casually
with ordinary application data.

## Summary

The intended minimal design is:

- OTP stores a per-device root key and immutable identity/lifecycle data
- the filesystem keeps active and, when needed, staging entitlement blobs
- official firmware updates are signed and encrypted
- the current root key + entitlement chain decrypts the current update
- an update may stage a new entitlement for future updates
- developer unlock erases the entitlement and permanently disables official encrypted updates

That gives UniLANCE:

- firmware confidentiality at rest
- authenticity for official updates
- per-device identity and lifecycle control via OTP
- model-batch confidentiality management for official update blobs
- revocable update entitlement
- a real irreversible unlock path for custom firmware and debug access
