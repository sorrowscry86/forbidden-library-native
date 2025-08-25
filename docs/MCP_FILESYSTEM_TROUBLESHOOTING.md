# MCP Filesystem-with-Morph Troubleshooting Guide

## Common Issues and Solutions

### 1. Path-Related Errors

#### Issue: "File not found" or "Path does not exist"
**Symptoms**: Operations fail with file/path not found errors
**Causes**:
- Using relative paths instead of absolute paths
- Incorrect path format for operating system
- File doesn't actually exist
- Path contains special characters

**Solutions**:
```json
// ✅ Correct - Use absolute paths
{"path": "/absolute/path/to/file.txt"}

// ❌ Incorrect - Relative paths may fail
{"path": "./file.txt"}
{"path": "file.txt"}
```

**Diagnostic Steps**:
1. Check allowed directories first
2. Verify file exists with `list_directory`
3. Use `get_file_info` to validate path
4. Ensure path uses correct separators for OS

#### Issue: "Permission denied" or "Access denied"
**Symptoms**: Operations fail with permission errors
**Causes**:
- File is read-only
- Insufficient permissions
- File is locked by another process
- Path is outside allowed directories

**Solutions**:
```json
// 1. Check allowed directories
{"random_string": "check"}

// 2. Verify file permissions
{"path": "/absolute/path/file.txt"}

// 3. Try creating backup instead of direct edit
{"path": "/absolute/path/file.txt.backup", "content": "..."}
```

### 2. File Operation Errors

#### Issue: "Cannot overwrite existing file"
**Symptoms**: `write_file` fails when file exists
**Causes**:
- File already exists and operation doesn't support overwriting
- File is in use by another process

**Solutions**:
```json
// 1. Check if file exists first
{"path": "/absolute/path/file.txt"}

// 2. Use edit_file for modifications instead of write_file
{
  "path": "/absolute/path/file.txt",
  "code_edit": "// ... existing code ...\n// New content\n// ... existing code ...",
  "instruction": "Add new content to existing file"
}

// 3. Create backup before overwriting
{"path": "/absolute/path/file.txt.backup", "content": "..."}
```

#### Issue: "Invalid file content" or encoding errors
**Symptoms**: File operations fail with content-related errors
**Causes**:
- Binary file being treated as text
- Encoding issues with special characters
- Invalid JSON or syntax in content

**Solutions**:
```json
// 1. Check file type before reading
{"path": "/absolute/path/file.txt"}

// 2. Use proper escaping for special characters
{
  "path": "/absolute/path/file.txt",
  "content": "Line 1\nLine 2\nLine 3"
}

// 3. Validate JSON before writing
{
  "path": "/absolute/path/config.json",
  "content": "{\n  \"key\": \"value\",\n  \"number\": 42\n}"
}
```

### 3. Directory Operation Errors

#### Issue: "Directory not empty" when moving
**Symptoms**: `move_file` fails when moving directories with contents
**Causes**:
- Directory contains files or subdirectories
- Operation doesn't support recursive moves

**Solutions**:
```json
// 1. Check directory contents first
{"path": "/absolute/path/source-directory"}

// 2. Move files individually
{"source": "/absolute/path/source/file1.txt", "destination": "/absolute/path/dest/file1.txt"}
{"source": "/absolute/path/source/file2.txt", "destination": "/absolute/path/dest/file2.txt"}

// 3. Remove empty directory after moving contents
// (Use appropriate system command if available)
```

#### Issue: "Cannot create directory" errors
**Symptoms**: `create_directory` fails
**Causes**:
- Parent directory doesn't exist
- Insufficient permissions
- Path is outside allowed directories

**Solutions**:
```json
// 1. Create parent directories first
{"path": "/absolute/path/parent"}
{"path": "/absolute/path/parent/child"}

// 2. Check permissions and allowed directories
{"random_string": "check"}
```

### 4. Search and Pattern Matching Issues

#### Issue: Search returns no results
**Symptoms**: `search_files` returns empty results
**Causes**:
- Pattern doesn't match any files
- Search path is incorrect
- Case sensitivity issues
- Files are in excluded patterns

**Solutions**:
```json
// 1. Use broader patterns
{"path": "/absolute/path", "pattern": "*.js"}
{"path": "/absolute/path", "pattern": "config"}

// 2. Check directory structure first
{"path": "/absolute/path"}

// 3. Use case-insensitive patterns
{"path": "/absolute/path", "pattern": "Config"}
```

#### Issue: Search times out or is slow
**Symptoms**: Search operations take too long or timeout
**Causes**:
- Large directory trees
- Complex patterns
- Too many files to search

**Solutions**:
```json
// 1. Search in smaller, more specific directories
{"path": "/absolute/path/src", "pattern": "*.js"}

// 2. Use more specific patterns
{"path": "/absolute/path", "pattern": "main.js"}

// 3. Exclude large directories
{"path": "/absolute/path", "pattern": "*.js", "excludePatterns": ["node_modules", "dist"]}
```

### 5. Performance Issues

#### Issue: Reading large files is slow
**Symptoms**: `read_file` operations take a long time
**Causes**:
- Files are very large
- Reading entire file when only part is needed

**Solutions**:
```json
// 1. Use head/tail parameters
{"path": "/absolute/path/large-file.txt", "head": 100}
{"path": "/absolute/path/large-file.txt", "tail": 100}

// 2. Read specific sections
{"path": "/absolute/path/large-file.txt", "head": 50}
{"path": "/absolute/path/large-file.txt", "head": 100}
```

#### Issue: Multiple file operations are slow
**Symptoms**: Batch operations take too long
**Causes**:
- Reading files individually instead of using batch operations
- Too many operations at once

**Solutions**:
```json
// 1. Use read_multiple_files for batch reading
{
  "paths": [
    "/absolute/path/file1.txt",
    "/absolute/path/file2.txt",
    "/absolute/path/file3.txt"
  ]
}

// 2. Process files in smaller batches
// 3. Use directory listing to plan operations efficiently
```

### 6. Edit Operation Issues

#### Issue: Edit operations fail or produce unexpected results
**Symptoms**: `edit_file` doesn't work as expected
**Causes**:
- Insufficient context around changes
- Incorrect use of `// ... existing code ...` markers
- Changes conflict with existing content

**Solutions**:
```json
// 1. Include more context around changes
{
  "path": "/absolute/path/file.js",
  "code_edit": "function existingFunction() {\n  // ... existing code ...\n}\n\n// New function\nfunction newFunction() {\n  console.log('Hello');\n}\n\n// ... existing code ...",
  "instruction": "Add new function after existing function"
}

// 2. Use dryRun to preview changes
{
  "path": "/absolute/path/file.js",
  "code_edit": "// ... existing code ...\n// New content\n// ... existing code ...",
  "instruction": "Add new content",
  "dryRun": true
}

// 3. Make smaller, more focused edits
```

#### Issue: Tiny edit operations fail
**Symptoms**: `tiny_edit_file` doesn't work
**Causes**:
- Using for large edits instead of small changes
- Incorrect line matching
- Changes are too complex

**Solutions**:
```json
// 1. Use for single line or small changes only
{
  "path": "/absolute/path/file.txt",
  "edits": [
    {
      "oldText": "old value",
      "newText": "new value"
    }
  ]
}

// 2. Use edit_file for larger changes
// 3. Ensure exact line matching
```

### 7. Security and Access Issues

#### Issue: Operations fail due to security restrictions
**Symptoms**: All operations fail with access denied
**Causes**:
- Path is outside allowed directories
- Security policies block operations
- Insufficient permissions

**Solutions**:
```json
// 1. Always check allowed directories first
{"random_string": "check"}

// 2. Work within permitted boundaries
// 3. Use relative paths within allowed directories
// 4. Request permission for additional directories if needed
```

### 8. Integration Issues

#### Issue: Tools don't work with specific file types
**Symptoms**: Operations fail on certain file types
**Causes**:
- Binary files being treated as text
- Unsupported file formats
- Encoding issues

**Solutions**:
```json
// 1. Check file type before operations
{"path": "/absolute/path/file.bin"}

// 2. Handle binary files appropriately
// 3. Use proper encoding for text files
// 4. Avoid operations on unsupported file types
```

## Diagnostic Workflow

### Step-by-Step Troubleshooting Process

1. **Check Security Boundaries**
   ```json
   {"random_string": "check"}
   ```

2. **Verify Path Exists**
   ```json
   {"path": "/absolute/path/to/check"}
   ```

3. **Check File Information**
   ```json
   {"path": "/absolute/path/file.txt"}
   ```

4. **Test with Simple Operation**
   ```json
   {"path": "/absolute/path/file.txt", "head": 10}
   ```

5. **Use Dry Run for Edits**
   ```json
   {
     "path": "/absolute/path/file.txt",
     "code_edit": "// ... existing code ...\n// Test edit\n// ... existing code ...",
     "instruction": "Test edit operation",
     "dryRun": true
   }
   ```

## Error Recovery Strategies

### For File Reading Issues
1. Check if file exists
2. Verify path is absolute
3. Try reading with head/tail parameters
4. Check file permissions

### For File Writing Issues
1. Verify target directory exists
2. Check write permissions
3. Create backup before overwriting
4. Use edit_file instead of write_file for modifications

### For Directory Issues
1. Create parent directories first
2. Check directory contents before moving
3. Handle files individually if needed
4. Verify permissions

### For Search Issues
1. Use broader patterns
2. Search in smaller directories
3. Check for excluded patterns
4. Verify search path exists

## Best Practices for Error Prevention

1. **Always check allowed directories first**
2. **Use absolute paths consistently**
3. **Verify file existence before operations**
4. **Use dryRun for edit operations**
5. **Handle large files with head/tail parameters**
6. **Use batch operations when possible**
7. **Create backups before major changes**
8. **Test operations on small files first**

## Getting Help

When troubleshooting fails:
1. Document the exact error message
2. Note the operation that failed
3. Check if the issue is reproducible
4. Verify all prerequisites are met
5. Consider alternative approaches
6. Report issues with full context

This troubleshooting guide should help resolve most common issues encountered when using the filesystem-with-morph MCP tools. Always start with the diagnostic workflow and follow the error recovery strategies for the best results.
