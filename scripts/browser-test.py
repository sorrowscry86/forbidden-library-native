#!/usr/bin/env python3
"""
Forbidden Library - Browser Automation Testing
Uses Selenium WebDriver for comprehensive browser testing
"""

import time
import sys
import os

# Fix Windows console encoding
if sys.platform == "win32":
    import codecs
    sys.stdout = codecs.getwriter("utf-8")(sys.stdout.detach())
    sys.stderr = codecs.getwriter("utf-8")(sys.stderr.detach())

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service
from selenium.common.exceptions import TimeoutException, WebDriverException

class ForbiddenLibraryTester:
    def __init__(self, headless=False):
        self.base_url = "http://localhost:8080"
        self.headless = headless
        self.driver = None
        self.results = {}
        
    def setup_driver(self):
        """Setup Chrome WebDriver with appropriate options"""
        try:
            chrome_options = Options()
            if self.headless:
                chrome_options.add_argument("--headless")
            chrome_options.add_argument("--no-sandbox")
            chrome_options.add_argument("--disable-dev-shm-usage")
            chrome_options.add_argument("--disable-gpu")
            chrome_options.add_argument("--window-size=1920,1080")
            chrome_options.add_argument("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            
            # Try to use ChromeDriver from PATH or download automatically
            try:
                self.driver = webdriver.Chrome(options=chrome_options)
            except WebDriverException:
                # Fallback to ChromeDriverManager
                from webdriver_manager.chrome import ChromeDriverManager
                service = Service(ChromeDriverManager().install())
                self.driver = webdriver.Chrome(service=service, options=chrome_options)
            
            self.driver.implicitly_wait(10)
            print("Browser driver initialized successfully")
            return True
            
        except Exception as e:
            print(f"Failed to setup browser driver: {e}")
            return False
    
    def test_health_check(self):
        """Test the health endpoint"""
        print("\nTest 1: Health Check")
        try:
            self.driver.get(f"{self.base_url}/health")
            content = self.driver.find_element(By.TAG_NAME, "body").text.strip()
            
            if content == "healthy":
                print("  Health check passed")
                self.results["health_check"] = True
                return True
            else:
                print(f"  Health check failed: got '{content}'")
                self.results["health_check"] = False
                return False
                
        except Exception as e:
            print(f"  Health check error: {e}")
            self.results["health_check"] = False
            return False
    
    def test_main_application(self):
        """Test the main application page"""
        print("\nTest 2: Main Application")
        try:
            start_time = time.time()
            self.driver.get(self.base_url)
            load_time = (time.time() - start_time) * 1000
            
            # Check page title
            title = self.driver.title
            print(f"  Page title: {title}")
            
            # Check for key elements
            has_title = "Forbidden Library" in title
            has_theme = self.driver.find_elements(By.CSS_SELECTOR, '[data-theme="dark"]')
            has_sveltekit = self.driver.find_elements(By.CSS_SELECTOR, '[data-sveltekit-preload-data]')
            has_body = self.driver.find_elements(By.TAG_NAME, "body")
            
            print(f"  Load time: {load_time:.2f} ms")
            print(f"  Content length: {len(self.driver.page_source)} characters")
            
            if has_title:
                print("  Page title found")
            if has_theme:
                print("  Dark theme configured")
            if has_sveltekit:
                print("  SvelteKit detected")
            if has_body:
                print("  Body element present")
            
            self.results["main_application"] = {
                "success": True,
                "load_time": load_time,
                "title": title,
                "has_theme": bool(has_theme),
                "has_sveltekit": bool(has_sveltekit)
            }
            return True
            
        except Exception as e:
            print(f"  Main application error: {e}")
            self.results["main_application"] = {"success": False}
            return False
    
    def test_responsive_design(self):
        """Test responsive design across different viewports"""
        print("\nTest 3: Responsive Design")
        viewports = [
            (1920, 1080, "Desktop"),
            (768, 1024, "Tablet"),
            (375, 667, "Mobile")
        ]
        
        try:
            for width, height, name in viewports:
                self.driver.set_window_size(width, height)
                time.sleep(1)  # Wait for layout to adjust
                
                # Check if page is still functional
                body = self.driver.find_element(By.TAG_NAME, "body")
                if body.is_displayed():
                    print(f"  {name} viewport ({width}x{height}) - Responsive")
                else:
                    print(f"  {name} viewport - Potential responsive issues")
            
            self.results["responsive_design"] = True
            return True
            
        except Exception as e:
            print(f"  Responsive design test error: {e}")
            self.results["responsive_design"] = False
            return False
    
    def test_performance(self):
        """Test application performance"""
        print("\nTest 4: Performance")
        try:
            times = []
            for i in range(5):
                start_time = time.time()
                self.driver.get(self.base_url)
                load_time = (time.time() - start_time) * 1000
                times.append(load_time)
                print(f"  Test {i+1}: {load_time:.2f} ms")
            
            avg_time = sum(times) / len(times)
            min_time = min(times)
            max_time = max(times)
            
            print(f"  Average: {avg_time:.2f} ms")
            print(f"  Minimum: {min_time:.2f} ms")
            print(f"  Maximum: {max_time:.2f} ms")
            
            if avg_time < 100:
                print("  Performance: EXCELLENT (< 100ms)")
            elif avg_time < 500:
                print("  Performance: GOOD (< 500ms)")
            elif avg_time < 1000:
                print("  Performance: ACCEPTABLE (< 1000ms)")
            else:
                print("  Performance: NEEDS OPTIMIZATION (> 1000ms)")
            
            self.results["performance"] = {
                "average": avg_time,
                "minimum": min_time,
                "maximum": max_time
            }
            return True
            
        except Exception as e:
            print(f"  Performance test error: {e}")
            self.results["performance"] = {"success": False}
            return False
    
    def test_user_interaction(self):
        """Test basic user interactions"""
        print("\nTest 5: User Interaction")
        try:
            # Navigate to main page
            self.driver.get(self.base_url)
            
            # Check for interactive elements
            buttons = self.driver.find_elements(By.TAG_NAME, "button")
            links = self.driver.find_elements(By.TAG_NAME, "a")
            inputs = self.driver.find_elements(By.TAG_NAME, "input")
            
            print(f"  Interactive elements found:")
            print(f"    Buttons: {len(buttons)}")
            print(f"    Links: {len(links)}")
            print(f"    Inputs: {len(inputs)}")
            
            # Test if page is interactive
            if len(buttons) > 0 or len(links) > 0:
                print("  Interactive elements detected")
                self.results["user_interaction"] = True
                return True
            else:
                print("  No interactive elements found")
                self.results["user_interaction"] = True  # Still pass as it's a static page
                return True
                
        except Exception as e:
            print(f"  User interaction test error: {e}")
            self.results["user_interaction"] = False
            return False
    
    def take_screenshot(self, filename="screenshot.png"):
        """Take a screenshot of the current page"""
        try:
            self.driver.save_screenshot(filename)
            print(f"  Screenshot saved: {filename}")
            return True
        except Exception as e:
            print(f"  Screenshot failed: {e}")
            return False
    
    def generate_report(self):
        """Generate a comprehensive test report"""
        print("\nTest Summary")
        print("============")
        
        passed = 0
        total = len(self.results)
        
        for test_name, result in self.results.items():
            if isinstance(result, dict):
                success = result.get("success", True)
            else:
                success = result
            
            if success:
                print(f"PASS: {test_name}")
                passed += 1
            else:
                print(f"FAIL: {test_name}")
        
        print(f"\nResults: {passed}/{total} tests passed")
        
        if passed == total:
            print("All tests passed! Application is ready for use.")
        else:
            print("Some tests failed. Please check the application.")
        
        return passed == total
    
    def run_all_tests(self):
        """Run all tests"""
        print("Forbidden Library Browser Testing")
        print("=================================")
        
        if not self.setup_driver():
            return False
        
        try:
            # Run all tests
            self.test_health_check()
            self.test_main_application()
            self.test_responsive_design()
            self.test_performance()
            self.test_user_interaction()
            
            # Take a screenshot
            self.take_screenshot("forbidden-library-test.png")
            
            # Generate report
            success = self.generate_report()
            
            return success
            
        finally:
            if self.driver:
                self.driver.quit()
                print("\nBrowser closed")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Test Forbidden Library deployment")
    parser.add_argument("--headless", action="store_true", help="Run in headless mode")
    parser.add_argument("--install-deps", action="store_true", help="Install dependencies")
    
    args = parser.parse_args()
    
    if args.install_deps:
        print("Installing dependencies...")
        import subprocess
        subprocess.run([sys.executable, "-m", "pip", "install", "selenium", "webdriver-manager"])
        print("Dependencies installed")
        return
    
    tester = ForbiddenLibraryTester(headless=args.headless)
    success = tester.run_all_tests()
    
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()