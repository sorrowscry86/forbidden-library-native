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

- [VOI-61: Cross-Platform QA: Windows/macOS/Linux](https://linear.app/voidcatrdc/issue/VOI-61/cross-platform-qa-windowsmacoslinux)
- [VOI-62: UAT: Flow-of-Thought Workflows](https://linear.app/voidcatrdc/issue/VOI-62/uat-flow-of-thought-workflows)
- [VOI-63: Prepare Signed Release Candidates (RC)](https://linear.app/voidcatrdc/issue/VOI-63/prepare-signed-release-candidates-rc)
- [VOI-64: Frontend Sentry: WebView Instrumentation](https://linear.app/voidcatrdc/issue/VOI-64/frontend-sentry-webview-instrumentation)
