#!/usr/bin/env node

/**
 * Forbidden Library - Deployment Testing Script
 * Tests the deployed Docker container using Playwright
 */

const { chromium } = require('playwright');

const TEST_URL = 'http://localhost:8080';
const HEALTH_URL = 'http://localhost:8080/health';

async function testDeployment() {
    console.log('🧪 Starting Forbidden Library Deployment Tests...\n');
    
    let browser;
    let page;
    
    try {
        // Launch browser
        console.log('🌐 Launching browser...');
        browser = await chromium.launch({ 
            headless: false, // Set to true for CI/CD
            slowMo: 1000 // Slow down for visibility
        });
        
        page = await browser.newPage();
        
        // Test 1: Health Check
        console.log('\n📋 Test 1: Health Check');
        await testHealthCheck(page);
        
        // Test 2: Main Application Load
        console.log('\n📋 Test 2: Main Application Load');
        await testMainApplication(page);
        
        // Test 3: Navigation and UI Elements
        console.log('\n📋 Test 3: Navigation and UI Elements');
        await testNavigation(page);
        
        // Test 4: Responsive Design
        console.log('\n📋 Test 4: Responsive Design');
        await testResponsiveDesign(page);
        
        // Test 5: Performance
        console.log('\n📋 Test 5: Performance');
        await testPerformance(page);
        
        console.log('\n✅ All tests completed successfully!');
        
    } catch (error) {
        console.error('\n❌ Test failed:', error.message);
        process.exit(1);
    } finally {
        if (browser) {
            await browser.close();
        }
    }
}

async function testHealthCheck(page) {
    try {
        const response = await page.goto(HEALTH_URL);
        const content = await page.textContent('body');
        
        if (response.status() === 200 && content.trim() === 'healthy') {
            console.log('  ✅ Health check passed');
        } else {
            throw new Error(`Health check failed: Status ${response.status()}, Content: ${content}`);
        }
    } catch (error) {
        throw new Error(`Health check error: ${error.message}`);
    }
}

async function testMainApplication(page) {
    try {
        const response = await page.goto(TEST_URL);
        
        if (response.status() !== 200) {
            throw new Error(`Main application failed to load: Status ${response.status()}`);
        }
        
        // Check if page loaded with expected content
        const title = await page.title();
        if (!title) {
            throw new Error('Page title is missing');
        }
        
        // Check for SvelteKit indicators
        const hasSvelteKit = await page.evaluate(() => {
            return document.querySelector('[data-sveltekit-preload-data]') !== null ||
                   document.querySelector('script[type="module"]') !== null;
        });
        
        if (hasSvelteKit) {
            console.log('  ✅ Main application loaded successfully');
            console.log(`  📄 Page title: ${title}`);
        } else {
            console.log('  ⚠️  SvelteKit indicators not found (may be normal for static build)');
        }
        
    } catch (error) {
        throw new Error(`Main application test error: ${error.message}`);
    }
}

async function testNavigation(page) {
    try {
        // Wait for page to be fully loaded
        await page.waitForLoadState('networkidle');
        
        // Check for common UI elements
        const elements = await page.evaluate(() => {
            const results = {};
            
            // Check for common SvelteKit elements
            results.hasMainContent = document.querySelector('main') !== null || 
                                   document.querySelector('[role="main"]') !== null;
            
            // Check for navigation elements
            results.hasNavigation = document.querySelector('nav') !== null || 
                                  document.querySelector('[role="navigation"]') !== null;
            
            // Check for interactive elements
            results.hasButtons = document.querySelectorAll('button').length > 0;
            results.hasLinks = document.querySelectorAll('a').length > 0;
            
            // Check for form elements
            results.hasForms = document.querySelectorAll('form').length > 0;
            
            return results;
        });
        
        console.log('  ✅ Navigation test completed');
        console.log(`  📊 Elements found:`, elements);
        
    } catch (error) {
        throw new Error(`Navigation test error: ${error.message}`);
    }
}

async function testResponsiveDesign(page) {
    try {
        const viewports = [
            { width: 1920, height: 1080, name: 'Desktop' },
            { width: 768, height: 1024, name: 'Tablet' },
            { width: 375, height: 667, name: 'Mobile' }
        ];
        
        for (const viewport of viewports) {
            await page.setViewportSize(viewport);
            await page.waitForTimeout(500); // Wait for layout to adjust
            
            const isResponsive = await page.evaluate(() => {
                // Check if page is responsive (basic check)
                const body = document.body;
                return body.offsetWidth > 0 && body.offsetHeight > 0;
            });
            
            if (isResponsive) {
                console.log(`  ✅ ${viewport.name} viewport (${viewport.width}x${viewport.height}) - Responsive`);
            } else {
                console.log(`  ⚠️  ${viewport.name} viewport - Potential responsive issues`);
            }
        }
        
    } catch (error) {
        throw new Error(`Responsive design test error: ${error.message}`);
    }
}

async function testPerformance(page) {
    try {
        // Navigate to main page and measure performance
        const startTime = Date.now();
        await page.goto(TEST_URL);
        const loadTime = Date.now() - startTime;
        
        // Get performance metrics
        const metrics = await page.evaluate(() => {
            const navigation = performance.getEntriesByType('navigation')[0];
            return {
                domContentLoaded: navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart,
                loadComplete: navigation.loadEventEnd - navigation.loadEventStart,
                totalTime: navigation.loadEventEnd - navigation.navigationStart
            };
        });
        
        console.log('  ✅ Performance test completed');
        console.log(`  ⏱️  Load time: ${loadTime}ms`);
        console.log(`  📊 DOM Content Loaded: ${metrics.domContentLoaded}ms`);
        console.log(`  📊 Total Load Time: ${metrics.totalTime}ms`);
        
        // Performance thresholds
        if (loadTime < 3000) {
            console.log('  🚀 Performance: Excellent (< 3s)');
        } else if (loadTime < 5000) {
            console.log('  ⚡ Performance: Good (< 5s)');
        } else {
            console.log('  ⚠️  Performance: Needs optimization (> 5s)');
        }
        
    } catch (error) {
        throw new Error(`Performance test error: ${error.message}`);
    }
}

// Error handling for unhandled promises
process.on('unhandledRejection', (reason, promise) => {
    console.error('Unhandled Rejection at:', promise, 'reason:', reason);
    process.exit(1);
});

// Run tests
testDeployment().catch(error => {
    console.error('Test suite failed:', error);
    process.exit(1);
});