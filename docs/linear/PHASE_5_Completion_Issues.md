# Linear Draft Issues - Phase 5 Completion

This file contains draft Linear issues to create for Phase 6 (cross-platform testing & release prep) and follow-ups from Phase 5 completion. Use the Linear API tools to push these as issues.

## Issue: Cross-Platform Full QA
- Title: Cross-Platform QA: Windows/macOS/Linux
- Team: QA / Pandora
- Priority: High
- Labels: testing, cross-platform, release
- Description: |
  Run comprehensive cross-platform QA covering Windows 11 (x64), macOS Sonoma (Apple Silicon & Intel), and Ubuntu 22.04 LTS (amd64).
  Tasks:
  - Verify window rendering, menus, and native integrations per platform.
  - Validate file system permissions, keychain/credentials handling, and native dialogs.
  - Perform performance profiling for startup time and UI frame rate on each platform.
  - Validate Sentry platform-specific events and symbolication workflows.
  - Record environment specs and results as attachments.

## Issue: User Acceptance Testing (Flow of Thought)
- Title: UAT: Flow-of-Thought Workflows
- Team: Product / Contractor
- Priority: High
- Labels: uat, user-testing
- Description: |
  Conduct UAT focused on the "flow of thought" — multi-step user scenarios combining file ingestion, AI persona analysis, and follow-up actions.
  Tasks:
  - Define 10 canonical user journeys and acceptance criteria.
  - Execute and log all steps with timings and observed UX friction.
  - Aggregate feedback and create bug/feature issues with reproducible steps.

## Issue: Release Candidate Packaging
- Title: Prepare Signed Release Candidates (RC)
- Team: Release Engineering / Albedo
- Priority: Medium
- Labels: release, packaging, signing
- Description: |
  Create signed, installable packages for target platforms (.msi, .dmg, .deb/AppImage). Ensure code-signing artifacts are managed securely.
  Tasks:
  - Configure CI for platform-specific packaging.
  - Validate installer integrity and checksum publication.
  - Document signing procedures and required secrets.

## Issue: Frontend Sentry Integration (Finish)
- Title: Frontend Sentry: Browser/Webview Instrumentation
- Team: Frontend / Albedo
- Priority: Medium
- Labels: monitoring, frontend
- Description: |
  Complete Sentry instrumentation for SvelteKit frontend: capture UI errors, performance metrics (component render times, virtual list render), and integrate with existing backend monitoring.
  Tasks:
  - Add Sentry SDK and source maps upload in build pipeline.
  - Validate Sentry events from the WebView environment.
  - Add opt-in/opt-out controls for crash reporting in the UI.

---

Notes:
- These drafts are ready to be created via the Linear API (`mcp_linear_create_issue`). Adjust assignees/team names to match your Linear workspace. Attach relevant documents from `docs/` as needed.

## Created Issues (Linear)

The following issues are live in the `VoidCatRDC` team and associated with the project
"Forbidden Library v1.0 Release - Phase 6 QA & Packaging":

- [VOI-66: Cross-Platform Testing: Windows/macOS/Linux full feature sweep](https://linear.app/voidcatrdc/issue/VOI-66/cross-platform-testing-windowsmacoslinux-full-feature-sweep)
- [VOI-68: UAT: Flow-of-Thought Workflows](https://linear.app/voidcatrdc/issue/VOI-68/user-acceptance-testing-uat-flow-of-thought-workflows)
- [VOI-70: Code signing & packaging for Windows/macOS/Linux](https://linear.app/voidcatrdc/issue/VOI-70/code-signing-and-packaging-for-windowsmacoslinux)
- [VOI-67: Frontend Sentry Integration (SvelteKit)](https://linear.app/voidcatrdc/issue/VOI-67/frontend-sentry-integration-sveltekit)

Subtasks created (2025-08-27) to operationalize the above:

- VOI-67 Sentry Integration
  - [VOI-76: VOI-67a: Add Sentry SDK to SvelteKit WebView and initialize](https://linear.app/voidcatrdc/issue/VOI-76/voi-67a-add-sentry-sdk-to-sveltekit-webview-and-initialize)
  - [VOI-77: VOI-67b: Configure CI to upload source maps to Sentry](https://linear.app/voidcatrdc/issue/VOI-77/voi-67b-configure-ci-to-upload-source-maps-to-sentry)
  - [VOI-78: VOI-67c: Add privacy controls (opt-in/out) for frontend telemetry](https://linear.app/voidcatrdc/issue/VOI-78/voi-67c-add-privacy-controls-opt-inout-for-frontend-telemetry)
  - [VOI-79: VOI-67d: Frontend Sentry validation harness (errors + performance)](https://linear.app/voidcatrdc/issue/VOI-79/voi-67d-frontend-sentry-validation-harness-errors-performance)

- VOI-66 Cross-Platform QA
  - [VOI-80: VOI-66a: Windows 11 full feature sweep and report](https://linear.app/voidcatrdc/issue/VOI-80/voi-66a-windows-11-full-feature-sweep-and-report)
  - [VOI-81: VOI-66b: macOS Sonoma (Intel/ARM) full feature sweep and report](https://linear.app/voidcatrdc/issue/VOI-81/voi-66b-macos-sonoma-intelarm-full-feature-sweep-and-report)
  - [VOI-82: VOI-66c: Ubuntu 22.04 LTS full feature sweep and report](https://linear.app/voidcatrdc/issue/VOI-82/voi-66c-ubuntu-2204-lts-full-feature-sweep-and-report)
  - [VOI-83: VOI-66d: Cross-platform performance profiling (startup + FPS)](https://linear.app/voidcatrdc/issue/VOI-83/voi-66d-cross-platform-performance-profiling-startup-fps)

- VOI-70 Packaging & Signing
  - [VOI-84: VOI-70a: Windows packaging & code signing (MSI/EXE)](https://linear.app/voidcatrdc/issue/VOI-84/voi-70a-windows-packaging-and-code-signing-msiexe)
  - [VOI-85: VOI-70b: macOS signing & notarization (DMG)](https://linear.app/voidcatrdc/issue/VOI-85/voi-70b-macos-signing-and-notarization-dmg)
  - [VOI-86: VOI-70c: Linux .deb and AppImage packaging & signing](https://linear.app/voidcatrdc/issue/VOI-86/voi-70c-linux-deb-and-appimage-packaging-and-signing)
  - [VOI-87: VOI-70d: CI release pipeline integration (multi-OS, secrets)](https://linear.app/voidcatrdc/issue/VOI-87/voi-70d-ci-release-pipeline-integration-multi-os-secrets)
  - [VOI-88: VOI-70e: Checksums & README Downloads updates](https://linear.app/voidcatrdc/issue/VOI-88/voi-70e-checksums-and-readme-downloads-updates)

Notes:
- Labels are standardized to workspace labels (Phase labels, Bug/Feature/etc.). The Phase 6 label has been added for consistency.
- Titles may differ slightly from the drafts to better match current release scope.

## Sync status (2025-08-27)

- Canonical project: [Forbidden Library (Phase 6)](https://linear.app/voidcatrdc/project/forbidden-library-phase-6-d55d9c47a500)
- Standardized labels created/verified: `Phase 6`, `testing`, `cross-platform`, `release`, `uat`, `user-testing`, `packaging`, `signing`, `monitoring`, `frontend`.
- Applied labels:
  - VOI-66: `Phase 6`, `testing`, `cross-platform`, `release`
  - VOI-68: `Phase 6`, `uat`, `user-testing`
  - VOI-70: `Phase 6`, `release`, `packaging`, `signing`
  - VOI-67: `Phase 6`, `monitoring`, `frontend`

Status changes today:
- VOI-67 moved to In Progress; subtasks VOI-76–VOI-79 created with acceptance criteria.
- VOI-66 moved to In Progress; subtasks VOI-80–VOI-83 created for OS-specific sweeps and perf.
- VOI-70 remains in Backlog; packaging/signing subtasks VOI-84–VOI-88 created.

Actionable next steps:
- Begin VOI-76: add `@sentry/sveltekit`, client-only init, and a validation route; then wire source maps (VOI-77).
- Kick off VOI-80 Windows sweep; prepare macOS/Linux checklists (VOI-81/82) and perf profiling (VOI-83).
