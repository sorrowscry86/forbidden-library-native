#!/usr/bin/env python3
"""
Debug script to examine the Forbidden Library page content
and identify any error messages
"""

from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By
import time

def debug_page():
    print("🔍 Debugging Forbidden Library Page")
    print("===================================")
    
    # Setup Chrome options
    options = Options()
    options.add_argument("--headless")
    options.add_argument("--no-sandbox")
    options.add_argument("--disable-dev-shm-usage")
    options.add_argument("--window-size=1920,1080")
    
    try:
        # Initialize driver
        driver = webdriver.Chrome(options=options)
        driver.implicitly_wait(10)
        
        print("✅ Browser driver initialized")
        
        # Navigate to the page
        print("🌐 Navigating to http://localhost:8080...")
        driver.get("http://localhost:8080")
        time.sleep(3)  # Wait for page to load
        
        print(f"📄 Page Title: '{driver.title}'")
        print(f"📄 Page Source Length: {len(driver.page_source)} characters")
        
        # Get body text
        try:
            body = driver.find_element(By.TAG_NAME, "body")
            body_text = body.text
            print(f"📄 Body Text Length: {len(body_text)} characters")
            print("📄 Body Text Preview:")
            print("-" * 50)
            print(body_text[:1000])
            print("-" * 50)
        except Exception as e:
            print(f"❌ Error getting body text: {e}")
        
        # Look for error messages
        print("\n🔍 Searching for error messages...")
        
        # Check for common error patterns
        error_patterns = [
            "error", "Error", "ERROR", "exception", "Exception", "EXCEPTION",
            "failed", "Failed", "FAILED", "not found", "Not Found", "404",
            "500", "502", "503", "timeout", "Timeout", "TIMEOUT"
        ]
        
        page_source = driver.page_source.lower()
        found_errors = []
        
        for pattern in error_patterns:
            if pattern.lower() in page_source:
                found_errors.append(pattern)
        
        if found_errors:
            print(f"⚠️  Found potential error patterns: {found_errors}")
        else:
            print("✅ No obvious error patterns found in page source")
        
        # Look for specific error elements
        error_elements = driver.find_elements(By.XPATH, 
            '//*[contains(text(), "error") or contains(text(), "Error") or contains(text(), "ERROR") or contains(text(), "exception") or contains(text(), "Exception")]')
        
        if error_elements:
            print(f"⚠️  Found {len(error_elements)} elements with error text:")
            for i, elem in enumerate(error_elements[:5]):  # Show first 5
                print(f"  {i+1}. {elem.text[:200]}")
        else:
            print("✅ No error elements found")
        
        # Check for console errors
        print("\n🔍 Checking for console errors...")
        logs = driver.get_log('browser')
        if logs:
            print(f"⚠️  Found {len(logs)} browser console entries:")
            for log in logs[:5]:  # Show first 5
                print(f"  {log['level']}: {log['message'][:200]}")
        else:
            print("✅ No browser console errors found")
        
        # Take a new screenshot
        screenshot_path = "debug-screenshot.png"
        driver.save_screenshot(screenshot_path)
        print(f"📸 Screenshot saved: {screenshot_path}")
        
        # Check if page is actually loading content
        print("\n🔍 Checking page content...")
        if len(driver.page_source) < 1000:
            print("⚠️  Page source seems very short, might indicate an error")
        else:
            print("✅ Page source length looks normal")
        
        # Check for specific content
        if "Forbidden Library" in driver.page_source:
            print("✅ 'Forbidden Library' text found in page")
        else:
            print("⚠️  'Forbidden Library' text NOT found in page")
        
        if "data-theme=\"dark\"" in driver.page_source:
            print("✅ Dark theme configuration found")
        else:
            print("⚠️  Dark theme configuration NOT found")
        
        if "sveltekit" in driver.page_source.lower():
            print("✅ SvelteKit references found")
        else:
            print("⚠️  SvelteKit references NOT found")
        
    except Exception as e:
        print(f"❌ Error during debugging: {e}")
    
    finally:
        if 'driver' in locals():
            driver.quit()
            print("🔚 Browser closed")

if __name__ == "__main__":
    debug_page()