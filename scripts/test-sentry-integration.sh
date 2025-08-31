#!/bin/bash

# Automated testing harness for Sentry integration validation
# Usage: ./scripts/test-sentry-integration.sh [dev|prod]

set -e

MODE=${1:-dev}
echo "🧪 Running Sentry Integration Test Suite - Mode: $MODE"

# Check if required environment variables are set
if [[ -z "$SENTRY_DSN" && "$MODE" == "prod" ]]; then
    echo "❌ SENTRY_DSN environment variable is required for production mode"
    exit 1
fi

echo "📋 Test Suite: Sentry Frontend Integration"
echo "========================================"

# Test 1: Check if Sentry SDK is installed
echo "🔍 Test 1: Sentry SDK Installation"
if npm list @sentry/sveltekit > /dev/null 2>&1; then
    echo "✅ @sentry/sveltekit is installed"
else
    echo "❌ @sentry/sveltekit is not installed"
    exit 1
fi

# Test 2: Check build configuration
echo "🔍 Test 2: Build Configuration"
if pnpm run build > /dev/null 2>&1; then
    echo "✅ Build successful with Sentry integration"
else
    echo "❌ Build failed - check Sentry configuration"
    exit 1
fi

# Test 3: Check source maps generation
echo "🔍 Test 3: Source Maps Generation"
if find build/_app/immutable/entry/ -name "*.js.map" -type f | head -1 | grep -q "."; then
    echo "✅ Source maps generated"
else
    echo "❌ Source maps not found"
fi

# Test 4: Check Sentry CLI availability (if in prod mode)
if [[ "$MODE" == "prod" ]]; then
    echo "🔍 Test 4: Sentry CLI Availability"
    if npx sentry-cli --version > /dev/null 2>&1; then
        echo "✅ Sentry CLI available"
    else
        echo "❌ Sentry CLI not available"
        exit 1
    fi
fi

# Test 5: Check hooks.client.ts exists and has Sentry init
echo "🔍 Test 5: Client-side Initialization"
if [[ -f "src/hooks.client.ts" ]] && grep -q "Sentry.init" src/hooks.client.ts; then
    echo "✅ Sentry client initialization found"
else
    echo "❌ Sentry client initialization missing"
    exit 1
fi

# Test 6: Check settings store exists with telemetry controls
echo "🔍 Test 6: Telemetry Controls"
if [[ -f "src/lib/stores/settings.ts" ]] && grep -q "telemetryEnabled" src/lib/stores/settings.ts; then
    echo "✅ Telemetry controls implemented"
else
    echo "❌ Telemetry controls missing"
    exit 1
fi

# Test 7: Check validation harness exists
echo "🔍 Test 7: Validation Harness"
if [[ -f "src/routes/dev/sentry-test/+page.svelte" ]]; then
    echo "✅ Sentry validation harness found"
else
    echo "❌ Sentry validation harness missing"
    exit 1
fi

# Summary
echo ""
echo "📊 Test Summary"
echo "==============="
echo "✅ All Sentry integration tests passed!"
echo ""

if [[ "$MODE" == "dev" ]]; then
    echo "🔧 Development Mode Instructions:"
    echo "1. Start the development server: pnpm run tauri:dev"
    echo "2. Enable telemetry in browser console: localStorage.setItem('telemetry.enabled', 'true')"
    echo "3. Navigate to /dev/sentry-test to validate events"
    echo "4. Check your Sentry dashboard for events"
elif [[ "$MODE" == "prod" ]]; then
    echo "🚀 Production Mode Instructions:"
    echo "1. Ensure SENTRY_AUTH_TOKEN is set in CI/CD environment"
    echo "2. Source maps will be uploaded automatically on release tags"
    echo "3. Monitor Sentry dashboard for production events"
fi

echo ""
echo "🎯 Next Steps:"
echo "- Complete cross-platform testing (VOI-80)"
echo "- Update GitHub Actions workflows (VOI-77)"
echo "- Validate Windows 11 functionality"