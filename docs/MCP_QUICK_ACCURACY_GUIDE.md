# MCP Coding Accuracy Quick Guide

## 🎯 Core Rules for AI Assistants

### Always Follow This Workflow:

1. **READ FIRST** - Always read the target file before editing
2. **USE ABSOLUTE PATHS** - Never use relative or tilde paths
3. **MAKE SMALL EDITS** - Prefer multiple small changes over one large edit
4. **PREVIEW CHANGES** - Use `dryRun: true` for complex edits
5. **VERIFY RESULTS** - Read the file after editing to confirm

### Required Pattern for Every Edit:

```json
// Step 1: Read the file first
{"path": "/absolute/path/to/file.js", "head": 100}

// Step 2: Make surgical edit with context
{
  "path": "/absolute/path/to/file.js",
  "code_edit": "// ... existing code ...\n// NEW CODE HERE\n// ... existing code ...",
  "instruction": "Specific description of what this edit does"
}

// Step 3: Verify the change
{"path": "/absolute/path/to/file.js", "head": 50}
```

### ⚠️ Critical Don'ts:

- ❌ Don't assume file structure - READ FIRST
- ❌ Don't use relative paths - ALWAYS ABSOLUTE
- ❌ Don't make large changes - BREAK INTO SMALL EDITS
- ❌ Don't ignore context - INCLUDE SURROUNDING CODE
- ❌ Don't skip verification - ALWAYS CONFIRM CHANGES

### 🔧 File-Specific Guidelines:

**JavaScript/TypeScript:**

- Preserve imports and function signatures
- Maintain existing comments and formatting
- Respect indentation

**Configuration Files:**

- Use `dryRun` for JSON/YAML changes
- Validate syntax before applying
- Preserve existing structure

**Documentation:**

- Maintain heading hierarchy
- Preserve links and references
- Keep consistent formatting

### 🎯 Success Checklist:

- [ ] File modified correctly
- [ ] Syntax is valid
- [ ] No unintended changes
- [ ] Context preserved
- [ ] Functionality maintained

**Remember**: When in doubt, read more context and make smaller changes. Precision over speed.
