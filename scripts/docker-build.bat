@echo off
setlocal enabledelayedexpansion

REM Docker build script for Forbidden Library (Windows)
echo 🐳 Building Forbidden Library Docker Container

REM Check if Docker is running
docker info >nul 2>&1
if errorlevel 1 (
    echo ❌ Docker is not running. Please start Docker and try again.
    exit /b 1
)

REM Build the production image
echo 📦 Building production image...
docker build -t forbidden-library:latest .
if errorlevel 1 (
    echo ❌ Failed to build production image
    exit /b 1
)
echo ✅ Production image built successfully!

REM Build the development image
echo 🔧 Building development image...
docker build -f Dockerfile.dev -t forbidden-library:dev .
if errorlevel 1 (
    echo ❌ Failed to build development image
    exit /b 1
)
echo ✅ Development image built successfully!

echo 🎉 All Docker images built successfully!
echo 📋 Available commands:
echo   Production: docker run -p 8080:80 forbidden-library:latest
echo   Development: docker run -p 1430:1430 -v %cd%:/app forbidden-library:dev
echo   Docker Compose: docker-compose up -d

pause