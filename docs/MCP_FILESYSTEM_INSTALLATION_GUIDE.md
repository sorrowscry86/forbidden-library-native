# Morph MCP Server Installation and Setup Guide

## Overview

The Morph MCP Server provides lightning-fast AI code editing capabilities through the Model Context Protocol (MCP). This guide covers complete installation, configuration, and setup for various MCP-compatible clients.

**Key Features:**
- **Lightning Fast**: 4500+ tokens/sec code editing
- **High Accuracy**: 98% success rate on code transformations
- **Flexible Tools**: Choose between edit-only or full filesystem access
- **Universal**: Works with Claude Desktop, Cursor, VS Code, and any MCP-compatible client

## Prerequisites

### Required Software
- **Node.js** (version 16 or higher)
- **npm** or **npx** (comes with Node.js)
- **Morph API Key** (get from [morphllm.com](https://morphllm.com/dashboard/api-keys))

### Supported Clients
- Claude Desktop
- Cursor
- VS Code (with MCP extension)
- Any MCP-compatible client

## Installation Steps

### Step 1: Get Your API Key

1. Visit [morphllm.com](https://morphllm.com/dashboard/api-keys)
2. Create an account or sign in
3. Generate a new API key
4. Copy the key (starts with `sk-`)

### Step 2: Choose Your Configuration Mode

#### Option A: Edit-Only Mode (Recommended for Security)
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

#### Option B: Full Access Mode
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

### Step 3: Configure Your Client

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

**Important**: Replace `your-api-key-here` with your actual API key and adjust the path to match your system.

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

### Step 4: Restart Your Client

**Critical**: You must completely restart your client (quit and reopen) for the MCP configuration to take effect.

## Workspace-Aware Configuration

The Morph MCP Server now supports **workspace-aware global configuration** by default, which automatically detects your current workspace root.

### How Workspace Mode Works

The server automatically detects workspace roots by looking for:
- `.git` directories
- `package.json`, `Cargo.toml`, `pyproject.toml`
- `.vscode`, `.cursor` directories
- Other common project files

### Recommended Global Configuration

For cross-project use, use this simplified configuration:

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

This configuration will automatically adapt to any project you open.

### Disabling Workspace Mode (Legacy)

If you need to disable workspace mode:

```json
{
  "env": {
    "MORPH_API_KEY": "your-api-key-here",
    "ALL_TOOLS": "true",
    "ENABLE_WORKSPACE_MODE": "false"
  }
}
```

## Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `MORPH_API_KEY` | Your Morph API key | Yes | None |
| `ALL_TOOLS` | Enable full filesystem access | No | "false" |
| `ENABLE_WORKSPACE_MODE` | Enable workspace detection | No | "true" |

## Testing Your Installation

### Step 1: Verify Package Installation

```bash
# Test if the package can be installed
npm install -g @morph-llm/morph-fast-apply

# Check if the package works
npx @morph-llm/morph-fast-apply --help
```

### Step 2: Test MCP Server

```bash
# Test edit-only mode
export MORPH_API_KEY="your-api-key-here"
export ALL_TOOLS="false"
npx @morph-llm/morph-fast-apply /path/to/your/project/

# Test full access mode
export ALL_TOOLS="true"
npx @morph-llm/morph-fast-apply /path/to/your/project/
```

### Step 3: Verify Client Integration

1. **List Tools**: Ask your AI assistant: "What MCP tools are available?"
2. **Test Edit**: Try: "Edit this file to add a comment at the top"
3. **Check Access**: If using `ALL_TOOLS: "true"`, try: "List the files in this directory"

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

## Troubleshooting

### Common Issues and Solutions

#### Tools Not Showing Up in Client

**Symptoms**: MCP tools don't appear in your AI assistant

**Solutions**:
1. Check that your client supports MCP servers
2. Verify your config file syntax is correct (JSON must be valid)
3. Restart your client completely (quit and reopen)
4. Check client logs for MCP-related errors
5. Verify the package can be installed: `npm install -g @morph-llm/morph-fast-apply`
6. Try asking your AI: "What MCP tools are available?"

#### API Key Errors

**Symptoms**: Authentication failures or API key errors

**Solutions**:
1. Verify your API key is correct in the environment variables
2. Ensure the key starts with 'sk-'
3. Check that the key has the right permissions
4. Get your API key from [morphllm.com](https://morphllm.com/dashboard/api-keys)
5. Test the key with a direct API call

#### File Access Issues

**Symptoms**: Cannot access files or directories

**Solutions**:
1. Check that the path in the config is correct
2. Verify you have read/write permissions to the directory
3. Try with `ALL_TOOLS: "false"` first to test basic editing
4. Check if the directory exists and is accessible

#### Package Installation Issues

**Symptoms**: Cannot install or run the MCP server

**Solutions**:
1. Ensure Node.js and npm are installed
2. Try installing globally: `npm install -g @morph-llm/morph-fast-apply`
3. Check npm permissions
4. Try running with `npx` instead of global install

### Diagnostic Workflow

If you're experiencing issues, follow this diagnostic workflow:

1. **Check Package Installation**:
   ```bash
   npm install -g @morph-llm/morph-fast-apply
   npx @morph-llm/morph-fast-apply --help
   ```

2. **Test API Key**:
   ```bash
   export MORPH_API_KEY="your-api-key-here"
   npx @morph-llm/morph-fast-apply /tmp/
   ```

3. **Verify Configuration**:
   - Check JSON syntax in your config file
   - Ensure all required fields are present
   - Verify paths are correct for your system

4. **Test Client Integration**:
   - Restart your client completely
   - Ask: "What MCP tools are available?"
   - Try a simple file operation

## Security Best Practices

### Recommended Security Settings

1. **Use Edit-Only Mode for Untrusted Environments**:
   ```json
   "ALL_TOOLS": "false"
   ```

2. **Limit Directory Scope**:
   ```json
   "args": ["@morph-llm/morph-fast-apply", "/specific/project/path/"]
   ```

3. **Regular Key Rotation**:
   - Rotate your API keys regularly
   - Monitor usage in your Morph dashboard

4. **Monitor Usage**:
   - Check your Morph dashboard for usage patterns
   - Set up alerts for unusual activity

### Security Considerations

- **API Key Security**: Never commit API keys to version control
- **Directory Access**: Limit access to only necessary directories
- **Permission Levels**: Use the minimum required permissions
- **Audit Trail**: Monitor file operations through your Morph dashboard

## Performance Optimization

### Best Practices for Performance

1. **Use Specific File Paths**: Specify exact file paths for faster operations
2. **Break Large Refactoring**: Split large changes into smaller, focused edits
3. **Monitor API Usage**: Keep track of your API usage and rate limits
4. **Cache Patterns**: Reuse successful editing patterns

### Performance Tips

- Use `head`/`tail` parameters for large files
- Use `read_multiple_files` for batch operations
- Use `dryRun` to preview changes before applying
- Use workspace mode for automatic project detection

## Support and Resources

### Official Resources

- **Documentation**: [Morph Apply API Docs](https://docs.morphllm.com/api-reference/endpoint/apply)
- **MCP Protocol**: [Model Context Protocol](https://modelcontextprotocol.io/)
- **Get API Key**: [morphllm.com](https://morphllm.com/dashboard/api-keys)
- **Support**: Contact at [morphllm.com](https://morphllm.com)

### Community Resources

- **GitHub**: Check for issues and updates
- **Discord**: Join the Morph community
- **Documentation**: Comprehensive guides and examples

## Verification Checklist

Before considering your installation complete, verify:

- [ ] API key is valid and working
- [ ] Package can be installed and run
- [ ] Configuration file is properly formatted
- [ ] Client has been restarted completely
- [ ] MCP tools appear in your AI assistant
- [ ] Basic file operations work
- [ ] Edit operations work as expected
- [ ] Security settings are appropriate for your environment

## Next Steps

Once your installation is complete and verified:

1. **Explore the Tools**: Try different file operations and editing capabilities
2. **Read the Documentation**: Review the comprehensive tool guides
3. **Practice with Examples**: Work through the practical examples
4. **Customize Configuration**: Adjust settings for your specific needs
5. **Monitor Usage**: Keep track of your API usage and performance

## Conclusion

The Morph MCP Server provides powerful, fast, and secure file editing capabilities through the Model Context Protocol. By following this installation guide and best practices, you'll have a robust setup that enhances your AI-assisted development workflow.

Remember to:
- Keep your API keys secure
- Monitor your usage
- Use appropriate security settings
- Restart your client after configuration changes
- Test your setup thoroughly

For additional help, refer to the troubleshooting section or contact Morph support.
