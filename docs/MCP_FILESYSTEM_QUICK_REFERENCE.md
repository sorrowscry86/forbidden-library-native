# MCP Filesystem-with-Morph Quick Reference

## Tool Summary

| Tool | Purpose | Key Parameters |
|------|---------|----------------|
| `read_file` | Read file contents | `path`, `head`, `tail` |
| `read_multiple_files` | Read multiple files | `paths[]` |
| `write_file` | Create/overwrite file | `path`, `content` |
| `edit_file` | Surgical file edits | `path`, `code_edit`, `instruction` |
| `create_directory` | Create directories | `path` |
| `list_directory` | List directory contents | `path` |
| `list_directory_with_sizes` | List with file sizes | `path`, `sortBy` |
| `directory_tree` | Recursive tree view | `path` |
| `move_file` | Move/rename files | `source`, `destination` |
| `search_files` | Find files by pattern | `path`, `pattern`, `excludePatterns[]` |
| `get_file_info` | Get file metadata | `path` |
| `list_allowed_directories` | Check access scope | `random_string` |
| `tiny_edit_file` | Line-based edits | `path`, `edits[]`, `dryRun` |

## Essential Commands

### Security Check
```json
{"random_string": "check"}
```

### Project Analysis
```json
{"path": "/absolute/project/path"}
{"path": "/absolute/project/path", "sortBy": "size"}
```

### File Reading
```json
{"path": "/absolute/path/file.txt", "head": 50}
{"paths": ["/path/file1.txt", "/path/file2.txt"]}
```

### File Writing
```json
{"path": "/absolute/path/newfile.txt", "content": "File content here"}
```

### File Editing
```json
{
  "path": "/absolute/path/file.js",
  "code_edit": "// ... existing code ...\n// New code\n// ... existing code ...",
  "instruction": "Add new function"
}
```

### Directory Operations
```json
{"path": "/absolute/path/newdir"}
{"path": "/absolute/path", "pattern": "*.js"}
```

## Best Practices

### ✅ Do
- Use absolute paths always
- Check allowed directories first
- Use `head`/`tail` for large files
- Use `read_multiple_files` for batch operations
- Use `dryRun` to preview edits
- Include context in `edit_file` operations

### ❌ Don't
- Use relative paths
- Assume file existence
- Read entire large files without limits
- Make large edits in single operations
- Skip security checks

## Common Patterns

### Project Setup
1. `list_allowed_directories`
2. `list_directory` (project root)
3. `create_directory` (if needed)
4. `write_file` (create files)

### Code Review
1. `list_directory` (src folder)
2. `read_file` with `head` (main files)
3. `search_files` (find related files)
4. `read_multiple_files` (batch read)

### Documentation
1. `read_file` (existing docs)
2. `edit_file` (update content)
3. `write_file` (create new docs)

## Error Recovery

| Error | Solution |
|-------|----------|
| File not found | Check path, verify existence |
| Permission denied | Check allowed directories |
| Invalid path | Use absolute paths |
| Directory not empty | Handle contents before move |

## Performance Tips

- Use `head`/`tail` for files > 1000 lines
- Batch related file reads with `read_multiple_files`
- Use `directory_tree` for complex project analysis
- Sort by size when analyzing disk usage
- Use `dryRun` before large edits

## Security Notes

- Only works within allowed directories
- Operations fail gracefully on access denial
- Paths are automatically normalized
- No access to system directories outside scope
