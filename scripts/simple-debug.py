#!/usr/bin/env python3
"""
Simple debug script to examine the Forbidden Library page
"""

from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By
import time

def debug_page():
    print("Debugging Forbidden Library Page")
    print("================================")
    
    options = Options()
    options.add_argument("--headless")
    options.add_argument("--no-sandbox")
    options.add_argument("--disable-dev-shm-usage")
    
    try:
        driver = webdriver.Chrome(options=options)
        driver.implicitly_wait(10)
        
        print("Browser driver initialized")
        
        driver.get("http://localhost:8080")
        time.sleep(3)
        
        print(f"Page Title: '{driver.title}'")
        print(f"Page Source Length: {len(driver.page_source)} characters")
        
        # Get body text
        body = driver.find_element(By.TAG_NAME, "body")
        body_text = body.text
        print(f"Body Text Length: {len(body_text)} characters")
        print("Body Text Preview:")
        print("-" * 50)
        print(body_text[:1000])
        print("-" * 50)
        
        # Look for error messages
        print("\nSearching for error messages...")
        
        error_elements = driver.find_elements(By.XPATH, 
            '//*[contains(text(), "error") or contains(text(), "Error") or contains(text(), "ERROR")]')
        
        if error_elements:
            print(f"Found {len(error_elements)} elements with error text:")
            for i, elem in enumerate(error_elements[:5]):
                print(f"  {i+1}. {elem.text}")
        else:
            print("No error elements found")
        
        # Take screenshot
        screenshot_path = "error-debug.png"
        driver.save_screenshot(screenshot_path)
        print(f"Screenshot saved: {screenshot_path}")
        
    except Exception as e:
        print(f"Error during debugging: {e}")
    
    finally:
        if 'driver' in locals():
            driver.quit()
            print("Browser closed")

if __name__ == "__main__":
    debug_page()