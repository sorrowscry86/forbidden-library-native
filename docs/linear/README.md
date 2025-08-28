Linear push guide

This directory contains draft issues for Phase 6. Use the activated Linear tools to create issues. Example usage (automated):

- Use `mcp_linear_create_issue` with the following fields:
  - `team`: team name or ID
  - `title`: issue title
  - `description`: markdown string
  - `labels`: array of label names
  - `priority`: 1..4 (optional)

Detected workspace team(s):

- `VoidCatRDC`

Canonical project:

- Forbidden Library (Phase 6): https://linear.app/voidcatrdc/project/forbidden-library-phase-6-d55d9c47a500

Notes:
- Older Linear projects have been archived in favor of the canonical project above.
- Use the `Phase 6` label for all issues in the current release scope.

Workflow:

1. Review `PHASE_5_Completion_Issues.md` and adjust titles/assignees to match your Linear workspace.
2. Use the `mcp_linear_create_issue` tool to create issues programmatically.
3. Capture returned issue IDs and insert them into this directory or the project checklist for traceability.

If you want, I can create the actual Linear issues now — tell me the team name(s) and assignee mapping and I will push them.

Standardized labels for Phase 6 tracking

- `Phase 6`
- `testing`, `cross-platform`, `release`
- `uat`, `user-testing`
- `packaging`, `signing`
- `monitoring`, `frontend`
