# Key Management Policy (Signing Keys)

This policy governs issuance, storage, use, rotation, and revocation of code signing and packaging keys for Forbidden Library.

## Scope

- Windows code signing certificate (.pfx)
- Apple Developer ID Application certificate (macOS)
- Apple App-Specific Password and Team ID (notarization)
- Linux GPG private keys for package signatures (.deb, AppImage)

## Issuance

- Keys issued to organization account owners only.
- Use reputable CAs (e.g., DigiCert) for Windows; Apple Developer Program for macOS.

## Storage

- Never commit secrets to the repo.
- Store keys in organization-approved secret managers or CI secrets.
- Limit access to Release Maintainers group.

## Use

- CI pulls keys only during release jobs.
- Local use requires explicit approval and secure environment.
- Logging must mask secrets.

## Rotation

- Rotate annually or upon suspected compromise.
- Maintain key identifiers and fingerprints in an internal registry.

## Revocation

- Revoke immediately upon compromise or role changes.
- Publish revocation notes and replace assets promptly.

## Audit

- Keep audit logs for key access in CI.
- Review semi-annually.
