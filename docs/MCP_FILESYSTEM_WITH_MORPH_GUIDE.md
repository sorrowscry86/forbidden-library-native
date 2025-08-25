# MCP Filesystem-with-Morph Tools Guide for MorphLLM

## Overview

The filesystem-with-morph MCP tools provide LLMs with secure, controlled access to the local file system through the Model Context Protocol (MCP). These tools enable intelligent file operations while maintaining security boundaries and providing detailed metadata about file operations.

**Key Features:**
- **Lightning Fast**: 4500+ tokens/sec code editing
- **High Accuracy**: 98% success rate on code transformations
- **Flexible Tools**: Choose between edit-only or full filesystem access
- **Universal**: Works with Claude Desktop, Cursor, VS Code, and any MCP-compatible client

## Installation and Configuration

### Quick Start

#### Claude Desktop Configuration
**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "filesystem-with-morph": {
      "command": "npx",
      "args": [
        "@morph-llm/morph-fast-apply",
        "/Users/your-username/"
      ],
      "env": {
        "MORPH_API_KEY": "your-api-key-here",
        "ALL_TOOLS": "true"
      }
    }
  }
}
```

#### Cursor Configuration
**Location**: `~/.cursor/mcp.json`

```json
{
  "mcpServers": {
    "filesystem-with-morph": {
      "command": "npx",
      "args": [
        "@morph-llm/morph-fast-apply",
        "/Users/your-username/"
      ],
      "env": {
        "MORPH_API_KEY": "your-api-key-here",
        "ALL_TOOLS": "true"
      }
    }
  }
}
```

#### VS Code Configuration
**Location**: `.vscode/mcp.json` in your workspace

```json
{
  "mcpServers": {
    "filesystem-with-morph": {
      "command": "npx",
      "args": [
        "@morph-llm/morph-fast-apply",
        "/Users/your-username/"
      ],
      "env": {
        "MORPH_API_KEY": "your-api-key-here",
        "ALL_TOOLS": "true"
      }
    }
  }
}
```

### Configuration Modes

#### 1. Edit-Only Mode (ALL_TOOLS: "false")
Fast file editing only with the `edit_file` tool:

```json
{
  "mcpServers": {
    "filesystem-with-morph": {
      "command": "npx",
      "args": ["@morph-llm/morph-fast-apply"],
      "env": {
        "MORPH_API_KEY": "your-api-key-here",
        "ALL_TOOLS": "false"
      }
    }
  }
}
```

#### 2. Full Access Mode (ALL_TOOLS: "true")
Complete filesystem access with all available tools:

```json
{
  "mcpServers": {
    "filesystem-with-morph": {
      "command": "npx",
      "args": ["@morph-llm/morph-fast-apply"],
      "env": {
        "MORPH_API_KEY": "your-api-key-here",
        "ALL_TOOLS": "true"
      }
    }
  }
}
```

### Workspace-Aware Global Configuration

The workspace mode is now **enabled by default** and automatically detects the current workspace root:

**Automatic Detection**: Looks for common indicators:
- `.git` directories
- `package.json`, `Cargo.toml`, `pyproject.toml`
- `.vscode`, `.cursor` directories
- Other common project files

**Recommended Global Config**:
```json
{
  "mcpServers": {
    "filesystem-with-morph": {
      "command": "npx",
      "args": ["@morph-llm/morph-fast-apply"],
      "env": {
        "MORPH_API_KEY": "your-api-key-here",
        "ALL_TOOLS": "true"
      }
    }
  }
}
```

### Environment Variables

- **MORPH_API_KEY**: Your Morph API key (required)
- **ALL_TOOLS**: Set to "true" for full filesystem access, "false" for edit-only mode
- **ENABLE_WORKSPACE_MODE**: Set to "false" to disable workspace mode (legacy behavior)

## Available Tools

### 1. File Reading Operations

#### `read_file`
**Purpose**: Read complete contents of a file from the file system
**Use Cases**: 
- Examining source code files
- Reading configuration files
- Analyzing log files
- Processing text-based data files

**Parameters**:
- `path` (required): Absolute path to the file
- `head` (optional): Read only first N lines
- `tail` (optional): Read only last N lines

**Best Practices**:
- Always use absolute paths for reliability
- Use `head` or `tail` for large files to avoid performance issues
- Handle encoding automatically (supports various text encodings)

**Example Usage**:
```json
{
  "path": "/absolute/path/to/file.txt",
  "head": 50
}
```

#### `read_multiple_files`
**Purpose**: Read contents of multiple files simultaneously
**Use Cases**:
- Comparing multiple files
- Batch processing of related files
- Analyzing project structure
- Cross-referencing documentation

**Parameters**:
- `paths` (required): Array of absolute file paths

**Best Practices**:
- More efficient than reading files one by one
- Failed reads for individual files won't stop the entire operation
- Use for related files that need to be analyzed together

### 2. File Writing Operations

#### `write_file`
**Purpose**: Create a new file or completely overwrite an existing file
**Use Cases**:
- Creating new source code files
- Generating configuration files
- Writing documentation
- Creating data files

**Parameters**:
- `path` (required): Absolute path where file should be created
- `content` (required): Text content to write

**Best Practices**:
- Use with caution as it overwrites existing files without warning
- Always use absolute paths
- Handle text content with proper encoding automatically

#### `edit_file`
**Purpose**: Make surgical edits to existing files (Lightning-fast code editing)
**Use Cases**:
- Adding new functions to existing code
- Updating configuration values
- Modifying documentation
- Refactoring code structure

**Parameters**:
- `path` (required): Absolute path to the file
- `code_edit` (required): Precise lines of code to edit
- `instruction` (required): Description of what the edit accomplishes
- `dryRun` (optional): Preview changes without applying

**Best Practices**:
- Use `// ... existing code ...` to represent unchanged sections
- Include sufficient context around changes for unambiguous application
- Make multiple small, focused edits rather than one large edit
- Use `dryRun` to preview changes before applying

### 3. Directory Operations

#### `create_directory`
**Purpose**: Create new directories or ensure directories exist
**Use Cases**:
- Setting up project structure
- Creating output directories
- Ensuring required paths exist
- Organizing file hierarchies

**Parameters**:
- `path` (required): Absolute path to create

**Best Practices**:
- Can create multiple nested directories in one operation
- Succeeds silently if directory already exists
- Perfect for setting up directory structures for projects

#### `list_directory`
**Purpose**: Get detailed listing of files and directories
**Use Cases**:
- Understanding project structure
- Finding specific files
- Analyzing directory contents
- Planning file operations

**Parameters**:
- `path` (required): Absolute path to list

**Best Practices**:
- Results clearly distinguish between files and directories
- Essential for understanding directory structure
- Use before performing file operations to understand context

#### `list_directory_with_sizes`
**Purpose**: Get detailed listing with file sizes
**Use Cases**:
- Analyzing disk usage
- Identifying large files
- Performance optimization
- Storage management

**Parameters**:
- `path` (required): Absolute path to list
- `sortBy` (optional): "name" or "size" sorting

**Best Practices**:
- Use for storage analysis and optimization
- Sort by size to identify largest files first
- Helpful for performance profiling

#### `directory_tree`
**Purpose**: Get recursive tree view as JSON structure
**Use Cases**:
- Complete project structure analysis
- Documentation generation
- Dependency mapping
- Architecture visualization

**Parameters**:
- `path` (required): Root path for tree generation

**Best Practices**:
- Returns structured JSON for programmatic analysis
- Each entry includes name, type, and children
- Useful for complex project analysis

### 4. File Management Operations

#### `move_file`
**Purpose**: Move or rename files and directories
**Use Cases**:
- Reorganizing project structure
- Renaming files
- Moving files between directories
- Refactoring file organization

**Parameters**:
- `source` (required): Absolute path to source file/directory
- `destination` (required): Absolute path to destination

**Best Practices**:
- Can move files between directories and rename in one operation
- Fails if destination already exists
- Works across different directories

#### `search_files`
**Purpose**: Recursively search for files matching patterns
**Use Cases**:
- Finding files when exact location is unknown
- Discovering related files
- Locating configuration files
- Finding documentation

**Parameters**:
- `path` (required): Starting path for search
- `pattern` (required): Case-insensitive pattern to match
- `excludePatterns` (optional): Patterns to exclude

**Best Practices**:
- Case-insensitive search with partial name matching
- Use for finding files when you don't know exact location
- Great for discovery and exploration

### 5. File Information Operations

#### `get_file_info`
**Purpose**: Retrieve detailed metadata about files and directories
**Use Cases**:
- Understanding file characteristics
- Performance analysis
- Security auditing
- File validation

**Parameters**:
- `path` (required): Absolute path to file/directory

**Best Practices**:
- Returns comprehensive metadata without reading content
- Perfect for understanding file characteristics
- Use before performing operations to validate files

#### `list_allowed_directories`
**Purpose**: Get list of directories accessible to the server
**Use Cases**:
- Understanding security boundaries
- Planning file operations
- Security validation
- Access verification

**Parameters**:
- `random_string` (required): Dummy parameter

**Best Practices**:
- Always check allowed directories before attempting operations
- Essential for understanding security scope
- Use to plan operations within permitted boundaries

### 6. Advanced Editing Operations

#### `tiny_edit_file`
**Purpose**: Make line-based edits to text files
**Use Cases**:
- Small, precise changes
- Configuration updates
- Bug fixes
- Minor refactoring

**Parameters**:
- `path` (required): Absolute path to file
- `edits` (required): Array of edit operations
- `dryRun` (optional): Preview changes

**Best Practices**:
- Use for single line or tiny edits only
- Each edit replaces exact line sequences
- Returns git-style diff showing changes
- Use `dryRun` to preview before applying

## Security and Best Practices

### Path Security
- **Always use absolute paths** for reliability
- Paths are automatically normalized regardless of slash direction
- Relative paths may fail as they depend on current working directory
- Tilde paths (~/...) might not work in all contexts

### Directory Access
- Only works within allowed directories
- Check `list_allowed_directories` before operations
- Respect security boundaries and permissions
- Operations fail gracefully if access is denied

### File Operations
- Handle text files normally
- Image files are returned as viewable images
- Supports various text encodings automatically
- Failed operations provide detailed error messages

### Performance Considerations
- Use `head`/`tail` for large files
- `read_multiple_files` is more efficient than individual reads
- Directory operations are optimized for large structures
- Search operations have configurable timeouts

## Common Usage Patterns

### 1. Project Analysis Workflow
```json
// 1. List allowed directories
{"random_string": "check"}

// 2. Explore project structure
{"path": "/absolute/project/path"}

// 3. Get detailed tree view
{"path": "/absolute/project/path"}

// 4. Read key configuration files
{"paths": ["/absolute/project/path/package.json", "/absolute/project/path/README.md"]}
```

### 2. Code Review Workflow
```json
// 1. List source directories
{"path": "/absolute/project/path/src"}

// 2. Read source files
{"path": "/absolute/project/path/src/main.js", "head": 100}

// 3. Search for related files
{"path": "/absolute/project/path", "pattern": "test"}

// 4. Read test files
{"paths": ["/absolute/project/path/tests/unit.js", "/absolute/project/path/tests/integration.js"]}
```

### 3. Documentation Generation Workflow
```json
// 1. Create documentation directory
{"path": "/absolute/project/path/docs"}

// 2. Read existing documentation
{"path": "/absolute/project/path/README.md"}

// 3. Create new documentation files
{"path": "/absolute/project/path/docs/API.md", "content": "# API Documentation\n\n..."}

// 4. Update existing files
{"path": "/absolute/project/path/README.md", "code_edit": "// ... existing code ...\n## New Section\n\nContent here\n// ... existing code ...", "instruction": "Add new documentation section"}
```

### 4. Configuration Management Workflow
```json
// 1. Read current configuration
{"path": "/absolute/project/path/config.json"}

// 2. Create backup
{"path": "/absolute/project/path/config.json.backup", "content": "..."}

// 3. Update configuration
{"path": "/absolute/project/path/config.json", "code_edit": "// ... existing code ...\n  \"newSetting\": \"value\",\n// ... existing code ...", "instruction": "Add new configuration setting"}
```

## Error Handling

### Common Error Scenarios
1. **File Not Found**: Check if file exists and path is correct
2. **Permission Denied**: Verify allowed directories and permissions
3. **Path Invalid**: Ensure absolute paths are used
4. **Directory Not Empty**: Handle when moving directories with contents

### Error Recovery Strategies
1. **Check allowed directories** before operations
2. **Use absolute paths** for all operations
3. **Verify file existence** before reading
4. **Handle failures gracefully** in batch operations

## Integration with MorphLLM

### Context Awareness
- Tools maintain context across operations
- File operations can inform subsequent AI responses
- Directory structure understanding enhances code generation
- File content analysis supports intelligent recommendations

### Workflow Integration
- Combine file operations with code generation
- Use file analysis for context-aware assistance
- Integrate with project management workflows
- Support documentation generation and maintenance

### Security Integration
- Respect MorphLLM security policies
- Work within defined access boundaries
- Provide audit trail for file operations
- Support secure development practices

## Advanced Techniques

### 1. Batch Processing
Use `read_multiple_files` for efficient batch operations:
```json
{
  "paths": [
    "/path/to/file1.js",
    "/path/to/file2.js", 
    "/path/to/file3.js"
  ]
}
```

### 2. Progressive Analysis
Start with directory listing, then drill down:
```json
// 1. Get overview
{"path": "/project/path"}

// 2. Focus on specific areas
{"path": "/project/path/src"}

// 3. Analyze specific files
{"path": "/project/path/src/main.js", "head": 50}
```

### 3. Safe Editing
Use `dryRun` to preview changes:
```json
{
  "path": "/project/path/file.js",
  "code_edit": "// ... existing code ...\n// New code here\n// ... existing code ...",
  "instruction": "Add new function",
  "dryRun": true
}
```

## Testing Your Setup

### Verification Steps
1. **Check Package**: Run `npx @morph-llm/morph-fast-apply --help`
2. **List Tools**: Ask your AI: "What MCP tools are available?"
3. **Test Edit**: Try a simple file edit operation
4. **Check Access**: If using `ALL_TOOLS: "true"`, try listing directory contents

### CLI Testing
```bash
# Install the package
npm install -g @morph-llm/morph-fast-apply

# Test edit-only mode
export MORPH_API_KEY="your-api-key-here"
export ALL_TOOLS="false"
npx @morph-llm/morph-fast-apply /path/to/your/project/

# Test full access mode
export ALL_TOOLS="true"
npx @morph-llm/morph-fast-apply /path/to/your/project/
```

## Troubleshooting

### Common Issues

**Tools not showing up in client:**
1. Check that your client supports MCP servers
2. Verify your config file syntax is correct (JSON must be valid)
3. Restart your client completely (quit and reopen)
4. Check client logs for MCP-related errors
5. Verify the package can be installed: `npm install -g @morph-llm/morph-fast-apply`
6. Try asking your AI: "What MCP tools are available?"

**API key errors:**
1. Verify your API key is correct in the environment variables
2. Ensure the key starts with 'sk-'
3. Check that the key has the right permissions
4. Get your API key from [morphllm.com](https://morphllm.com/dashboard/api-keys)
5. Test the key with a direct API call

**File access issues:**
1. Check that the path in the config is correct
2. Verify you have read/write permissions to the directory
3. Try with `ALL_TOOLS: "false"` first to test basic editing
4. Check if the directory exists and is accessible

**Package installation issues:**
1. Ensure Node.js and npm are installed
2. Try installing globally: `npm install -g @morph-llm/morph-fast-apply`
3. Check npm permissions
4. Try running with `npx` instead of global install

## Best Practices

### Security
- Use `ALL_TOOLS: "false"` for untrusted environments
- Limit the directory scope in your config
- Regularly rotate your API keys
- Monitor usage in your dashboard

### Performance
- Use specific file paths for faster operations
- Break large refactoring into smaller steps
- Monitor API usage and rate limits
- Cache frequently used patterns

## Advanced Configuration

### Custom Directory Scope
Limit the MCP server to specific directories:
```json
{
  "mcpServers": {
    "filesystem-with-morph": {
      "command": "npx",
      "args": [
        "@morph-llm/morph-fast-apply",
        "/Users/your-username/projects/specific-project/"
      ],
      "env": {
        "MORPH_API_KEY": "your-api-key-here",
        "ALL_TOOLS": "false"
      }
    }
  }
}
```

### Multiple Configurations
Run different MCP servers for different projects:
```json
{
  "mcpServers": {
    "morph-project-a": {
      "command": "npx",
      "args": ["@morph-llm/morph-fast-apply", "/path/to/project-a/"],
      "env": {
        "MORPH_API_KEY": "your-api-key-here",
        "ALL_TOOLS": "false"
      }
    },
    "morph-project-b": {
      "command": "npx",
      "args": ["@morph-llm/morph-fast-apply", "/path/to/project-b/"],
      "env": {
        "MORPH_API_KEY": "your-api-key-here",
        "ALL_TOOLS": "true"
      }
    }
  }
}
```

## Support and Resources

- **Documentation**: [Morph Apply API Docs](https://docs.morphllm.com/api-reference/endpoint/apply)
- **MCP Protocol**: [Model Context Protocol](https://modelcontextprotocol.io/)
- **Get API Key**: [morphllm.com](https://morphllm.com/dashboard/api-keys)
- **Support**: Contact at [morphllm.com](https://morphllm.com)

## Conclusion

The filesystem-with-morph MCP tools provide LLMs with powerful, secure file system access capabilities. By following these guidelines and best practices, LLMs can effectively assist users with file operations, code analysis, documentation generation, and project management while maintaining security and performance standards.

Remember to always use absolute paths, respect security boundaries, and leverage the tools' capabilities for efficient and intelligent file system operations within the MorphLLM environment.
