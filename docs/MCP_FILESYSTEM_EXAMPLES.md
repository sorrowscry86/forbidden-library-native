# MCP Filesystem-with-Morph Practical Examples

## Example 1: Project Initialization and Setup

### Scenario: Setting up a new React project structure

````json
// 1. Check access permissions
{"random_string": "check"}

// 2. Create project directory structure
{"path": "/absolute/path/react-project"}
{"path": "/absolute/path/react-project/src"}
{"path": "/absolute/path/react-project/src/components"}
{"path": "/absolute/path/react-project/src/pages"}
{"path": "/absolute/path/react-project/public"}
{"path": "/absolute/path/react-project/tests"}

// 3. Create package.json
{
  "path": "/absolute/path/react-project/package.json",
  "content": "{\n  \"name\": \"react-project\",\n  \"version\": \"1.0.0\",\n  \"dependencies\": {\n    \"react\": \"^18.0.0\",\n    \"react-dom\": \"^18.0.0\"\n  }\n}"
}

// 4. Create README.md
{
  "path": "/absolute/path/react-project/README.md",
  "content": "# React Project\n\nA new React application.\n\n## Setup\n\n```bash\nnpm install\nnpm start\n```"
}

// 5. Create main App component
{
  "path": "/absolute/path/react-project/src/App.js",
  "content": "import React from 'react';\n\nfunction App() {\n  return (\n    <div className=\"App\">\n      <h1>Hello World</h1>\n    </div>\n  );\n}\n\nexport default App;"
}
````

## Example 2: Code Review and Analysis

### Scenario: Reviewing a JavaScript project for potential improvements

```json
// 1. Explore project structure
{"path": "/absolute/path/js-project"}

// 2. Get detailed tree view
{"path": "/absolute/path/js-project"}

// 3. Read main source files
{"path": "/absolute/path/js-project/src/index.js", "head": 100}
{"path": "/absolute/path/js-project/src/utils.js", "head": 50}

// 4. Search for test files
{"path": "/absolute/path/js-project", "pattern": "test"}

// 5. Read test files
{"paths": ["/absolute/path/js-project/tests/index.test.js", "/absolute/path/js-project/tests/utils.test.js"]}

// 6. Check package.json for dependencies
{"path": "/absolute/path/js-project/package.json"}

// 7. Analyze file sizes to identify large files
{"path": "/absolute/path/js-project", "sortBy": "size"}
```

## Example 3: Documentation Generation

### Scenario: Creating comprehensive API documentation

```json
// 1. Read existing source code
{"path": "/absolute/path/api-project/src/api.js", "head": 200}

// 2. Read existing documentation
{"path": "/absolute/path/api-project/README.md"}

// 3. Create docs directory
{"path": "/absolute/path/api-project/docs"}

// 4. Create API documentation
{
  "path": "/absolute/path/api-project/docs/API.md",
  "content": "# API Documentation\n\n## Endpoints\n\n### GET /users\nReturns a list of users\n\n### POST /users\nCreates a new user\n\n## Authentication\n\nAll endpoints require Bearer token authentication."
}

// 5. Update main README with link to docs
{
  "path": "/absolute/path/api-project/README.md",
  "code_edit": "// ... existing code ...\n\n## Documentation\n\n- [API Documentation](./docs/API.md)\n\n// ... existing code ...",
  "instruction": "Add documentation section with link to API docs"
}
```

## Example 4: Configuration Management

### Scenario: Updating application configuration

```json
// 1. Read current configuration
{"path": "/absolute/path/app/config.json"}

// 2. Create backup
{
  "path": "/absolute/path/app/config.json.backup",
  "content": "{\n  \"database\": {\n    \"host\": \"localhost\",\n    \"port\": 5432\n  },\n  \"api\": {\n    \"timeout\": 5000\n  }\n}"
}

// 3. Update configuration with new settings
{
  "path": "/absolute/path/app/config.json",
  "code_edit": "{\n  \"database\": {\n    \"host\": \"localhost\",\n    \"port\": 5432\n  },\n  \"api\": {\n    \"timeout\": 10000,\n    \"retries\": 3\n  },\n  \"logging\": {\n    \"level\": \"info\",\n    \"file\": \"./logs/app.log\"\n  }\n}",
  "instruction": "Update configuration with new API timeout, retries, and logging settings"
}
```

## Example 5: File Organization and Refactoring

### Scenario: Reorganizing a messy project structure

```json
// 1. Analyze current structure
{"path": "/absolute/path/messy-project"}

// 2. Create organized directory structure
{"path": "/absolute/path/messy-project/src"}
{"path": "/absolute/path/messy-project/src/components"}
{"path": "/absolute/path/messy-project/src/utils"}
{"path": "/absolute/path/messy-project/assets"}
{"path": "/absolute/path/messy-project/docs"}

// 3. Move files to appropriate locations
{"source": "/absolute/path/messy-project/component1.js", "destination": "/absolute/path/messy-project/src/components/Component1.js"}
{"source": "/absolute/path/messy-project/component2.js", "destination": "/absolute/path/messy-project/src/components/Component2.js"}
{"source": "/absolute/path/messy-project/helper.js", "destination": "/absolute/path/messy-project/src/utils/helper.js"}
{"source": "/absolute/path/messy-project/logo.png", "destination": "/absolute/path/messy-project/assets/logo.png"}

// 4. Update import statements in moved files
{
  "path": "/absolute/path/messy-project/src/components/Component1.js",
  "code_edit": "// ... existing code ...\nimport { helper } from '../utils/helper.js';\n// ... existing code ...",
  "instruction": "Update import path to reflect new file location"
}
```

## Example 6: Batch File Processing

### Scenario: Processing multiple configuration files

```json
// 1. Find all config files
{"path": "/absolute/path/project", "pattern": "config"}

// 2. Read all config files at once
{
  "paths": [
    "/absolute/path/project/config.json",
    "/absolute/path/project/src/config.js",
    "/absolute/path/project/tests/test-config.json"
  ]
}

// 3. Create updated versions
{
  "path": "/absolute/path/project/config.json",
  "code_edit": "// ... existing code ...\n  \"version\": \"2.0.0\",\n// ... existing code ...",
  "instruction": "Update version number in main config"
}

{
  "path": "/absolute/path/project/src/config.js",
  "code_edit": "// ... existing code ...\nexport const VERSION = '2.0.0';\n// ... existing code ...",
  "instruction": "Add version constant to source config"
}
```

## Example 7: Error Handling and Recovery

### Scenario: Handling file operation failures gracefully

```json
// 1. Check if file exists before reading
{"path": "/absolute/path/project/src/main.js"}

// 2. If file doesn't exist, create it
{
  "path": "/absolute/path/project/src/main.js",
  "content": "// Main application entry point\nconsole.log('Application started');"
}

// 3. Verify file was created successfully
{"path": "/absolute/path/project/src/main.js"}

// 4. Create backup before making changes
{
  "path": "/absolute/path/project/src/main.js.backup",
  "content": "// Main application entry point\nconsole.log('Application started');"
}

// 5. Make changes with dry run first
{
  "path": "/absolute/path/project/src/main.js",
  "code_edit": "// Main application entry point\nimport { initialize } from './utils';\n\ninitialize();\nconsole.log('Application started');",
  "instruction": "Add initialization import and call",
  "dryRun": true
}
```

## Example 8: Performance Optimization

### Scenario: Analyzing and optimizing large codebases

```json
// 1. Get overview of project structure
{"path": "/absolute/path/large-project"}

// 2. Identify largest files
{"path": "/absolute/path/large-project", "sortBy": "size"}

// 3. Read large files in chunks
{"path": "/absolute/path/large-project/src/large-file.js", "head": 100}
{"path": "/absolute/path/large-project/src/large-file.js", "tail": 100}

// 4. Search for specific patterns
{"path": "/absolute/path/large-project", "pattern": "TODO"}
{"path": "/absolute/path/large-project", "pattern": "FIXME"}

// 5. Analyze specific directories
{"path": "/absolute/path/large-project/src/components"}

// 6. Create optimization report
{
  "path": "/absolute/path/large-project/OPTIMIZATION_REPORT.md",
  "content": "# Performance Optimization Report\n\n## Large Files Identified\n- src/large-file.js (2.5MB)\n\n## Recommendations\n1. Split large-file.js into smaller modules\n2. Remove unused imports\n3. Optimize bundle size\n\n## Action Items\n- [ ] Refactor large-file.js\n- [ ] Update build configuration\n- [ ] Add code splitting"
}
```

## Example 9: Security Audit

### Scenario: Performing security analysis of a project

```json
// 1. Check allowed directories
{"random_string": "check"}

// 2. Search for sensitive files
{"path": "/absolute/path/project", "pattern": "secret"}
{"path": "/absolute/path/project", "pattern": "key"}
{"path": "/absolute/path/project", "pattern": "password"}

// 3. Check for configuration files
{"path": "/absolute/path/project", "pattern": ".env"}

// 4. Analyze package.json for vulnerabilities
{"path": "/absolute/path/project/package.json"}

// 5. Check for hardcoded credentials
{"path": "/absolute/path/project/src", "pattern": "api_key"}

// 6. Create security report
{
  "path": "/absolute/path/project/SECURITY_AUDIT.md",
  "content": "# Security Audit Report\n\n## Findings\n- No hardcoded credentials found\n- Environment variables properly used\n- Dependencies up to date\n\n## Recommendations\n1. Enable 2FA for all accounts\n2. Regular dependency updates\n3. Code review for new features\n\n## Status: ✅ PASSED"
}
```

## Example 10: Continuous Integration Setup

### Scenario: Setting up CI/CD configuration files

```json
// 1. Create CI directory
{"path": "/absolute/path/project/.github"}
{"path": "/absolute/path/project/.github/workflows"}

// 2. Create main CI workflow
{
  "path": "/absolute/path/project/.github/workflows/ci.yml",
  "content": "name: CI\n\non:\n  push:\n    branches: [ main ]\n  pull_request:\n    branches: [ main ]\n\njobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n    - uses: actions/checkout@v3\n    - name: Install dependencies\n      run: npm install\n    - name: Run tests\n      run: npm test"
}

// 3. Create deployment workflow
{
  "path": "/absolute/path/project/.github/workflows/deploy.yml",
  "content": "name: Deploy\n\non:\n  push:\n    tags:\n      - 'v*'\n\njobs:\n  deploy:\n    runs-on: ubuntu-latest\n    steps:\n    - uses: actions/checkout@v3\n    - name: Deploy to production\n      run: echo 'Deploying...'"
}

// 4. Update README with CI status
{
  "path": "/absolute/path/project/README.md",
  "code_edit": "// ... existing code ...\n\n## CI/CD\n\nThis project uses GitHub Actions for continuous integration and deployment.\n\n- [CI Workflow](.github/workflows/ci.yml)\n- [Deploy Workflow](.github/workflows/deploy.yml)\n\n// ... existing code ...",
  "instruction": "Add CI/CD section to README"
}
```

These examples demonstrate practical, real-world usage patterns for the filesystem-with-morph MCP tools. Each example shows how to combine multiple tools to accomplish complex tasks while following best practices for security, performance, and error handling.
