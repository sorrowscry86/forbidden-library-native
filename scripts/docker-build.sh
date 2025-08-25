#!/bin/bash

# Docker build script for Forbidden Library
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🐳 Building Forbidden Library Docker Container${NC}"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}❌ Docker is not running. Please start Docker and try again.${NC}"
    exit 1
fi

# Build the production image
echo -e "${YELLOW}📦 Building production image...${NC}"
docker build -t forbidden-library:latest .

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Production image built successfully!${NC}"
else
    echo -e "${RED}❌ Failed to build production image${NC}"
    exit 1
fi

# Build the development image
echo -e "${YELLOW}🔧 Building development image...${NC}"
docker build -f Dockerfile.dev -t forbidden-library:dev .

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Development image built successfully!${NC}"
else
    echo -e "${RED}❌ Failed to build development image${NC}"
    exit 1
fi

echo -e "${GREEN}🎉 All Docker images built successfully!${NC}"
echo -e "${YELLOW}📋 Available commands:${NC}"
echo -e "  Production: docker run -p 8080:80 forbidden-library:latest"
echo -e "  Development: docker run -p 1430:1430 -v \$(pwd):/app forbidden-library:dev"
echo -e "  Docker Compose: docker-compose up -d"