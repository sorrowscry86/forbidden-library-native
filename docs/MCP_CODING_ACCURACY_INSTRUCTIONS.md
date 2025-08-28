# MCP Filesystem-with-Morph Coding Accuracy Instructions

## For AI Assistants: Improve Code Editing Accuracy

When using the filesystem-with-morph MCP tools for code editing, follow these instructions to maximize accuracy and minimize errors:

### 🎯 Core Principles

1. **Always use absolute paths** - Never use relative paths or tilde (~) paths
2. **Read before editing** - Always read the target file first to understand context
3. **Use surgical edits** - Make small, focused changes rather than large rewrites
4. **Preview changes** - Use `dryRun: true` for complex edits before applying
5. **Verify results** - Read the file after editing to confirm changes

### 📋 Required Workflow for Code Changes

#### Step 1: Context Gathering
```json
// Always read the file first
{
  "path": "/absolute/path/to/file.js",
  "head": 100  // Adjust based on file size
}
```

#### Step 2: Plan the Edit
- Identify the exact lines to modify
- Note surrounding context (3-5 lines before/after)
- Determine if multiple small edits are better than one large edit

#### Step 3: Execute with Precision
```json
{
  "path": "/absolute/path/to/file.js",
  "code_edit": "// ... existing code ...\n// NEW CODE HERE\n// ... existing code ...",
  "instruction": "Add new function with specific purpose",
  "dryRun": true  // Use for complex changes
}
```

#### Step 4: Verify the Result
```json
// Read the modified section to confirm
{
  "path": "/absolute/path/to/file.js",
  "head": 50
}
```

### 🔧 Best Practices for Different File Types

#### JavaScript/TypeScript Files
- Preserve existing import statements
- Maintain function signatures exactly
- Keep existing comments and documentation
- Respect indentation and formatting

#### Configuration Files (JSON, YAML, etc.)
- Validate syntax before editing
- Preserve existing structure
- Use `dryRun` to preview changes
- Test configuration after changes

#### Documentation Files (Markdown, etc.)
- Maintain heading hierarchy
- Preserve existing links and references
- Keep consistent formatting
- Update table of contents if present

### ⚠️ Common Pitfalls to Avoid

1. **Don't assume file structure** - Always read first
2. **Don't use relative paths** - Always use absolute paths
3. **Don't make large changes** - Break into smaller edits
4. **Don't ignore context** - Include sufficient surrounding code
5. **Don't skip verification** - Always confirm changes worked

### 🎯 High-Accuracy Edit Patterns

#### Pattern 1: Function Addition
```json
// 1. Read to find insertion point
{"path": "/file.js", "head": 100}

// 2. Add function with context
{
  "path": "/file.js",
  "code_edit": "// ... existing code ...\n\nfunction newFunction() {\n  // implementation\n}\n\n// ... existing code ...",
  "instruction": "Add new function after existing functions"
}
```

#### Pattern 2: Configuration Update
```json
// 1. Read current config
{"path": "/config.json"}

// 2. Update with dryRun
{
  "path": "/config.json",
  "code_edit": "{\n  // ... existing code ...\n  \"newSetting\": \"value\",\n  // ... existing code ...\n}",
  "instruction": "Add new configuration setting",
  "dryRun": true
}
```

#### Pattern 3: Import Statement Addition
```json
// 1. Read to see existing imports
{"path": "/file.js", "head": 20}

// 2. Add import with proper grouping
{
  "path": "/file.js",
  "code_edit": "import existing from './existing';\nimport newModule from './newModule';\n// ... existing code ...",
  "instruction": "Add new import statement with existing imports"
}
```

### 🔍 Verification Checklist

After each edit, verify:
- [ ] File was modified correctly
- [ ] Syntax is valid
- [ ] No unintended changes occurred
- [ ] Context is preserved
- [ ] Functionality is maintained

### 🚀 Advanced Techniques

#### For Large Refactoring
1. **Break into phases** - Make multiple small edits
2. **Use dryRun extensively** - Preview each phase
3. **Read between phases** - Verify intermediate state
4. **Test incrementally** - Validate each change

#### For Complex File Structures
1. **Read entire file first** - Understand full context
2. **Map dependencies** - Note imports and references
3. **Plan edit sequence** - Order changes logically
4. **Verify structure** - Check file integrity after changes

### 📝 Error Recovery

If an edit fails:
1. **Read the file** - Check current state
2. **Identify the issue** - Syntax, path, or context problem
3. **Fix incrementally** - Make smaller, targeted changes
4. **Verify each step** - Confirm before proceeding

### 🎯 Success Metrics

A successful edit should:
- ✅ Maintain file integrity
- ✅ Preserve existing functionality
- ✅ Follow project conventions
- ✅ Pass syntax validation
- ✅ Meet the user's requirements

---

**Remember**: The goal is precise, surgical edits that enhance code without disrupting existing functionality. When in doubt, read more context and make smaller changes.

