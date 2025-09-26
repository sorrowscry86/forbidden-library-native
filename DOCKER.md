# Docker Setup for Forbidden Library

This document provides instructions for building and running the Forbidden Library application using Docker.

## Prerequisites

- Docker Desktop installed and running
- Docker Compose (usually included with Docker Desktop)

## Quick Start

### Using Docker Compose (Recommended)

1. **Build and run the application:**

   ```bash
   docker-compose up -d
   ```

2. **Access the application:**
   - Production: http://localhost:8080
   - Development: http://localhost:1430 (if using dev profile)

3. **Stop the application:**
   ```bash
   docker-compose down
   ```

### Manual Docker Commands

#### Production Build

1. **Build the production image:**

   ```bash
   docker build -t forbidden-library:latest .
   ```

2. **Run the production container:**

   ```bash
   docker run -p 8080:80 forbidden-library:latest
   ```

3. **Access the application:**
   - http://localhost:8080

#### Development Build

1. **Build the development image:**

   ```bash
   docker build -f Dockerfile.dev -t forbidden-library:dev .
   ```

2. **Run the development container:**

   ```bash
   docker run -p 1430:1430 -v $(pwd):/app forbidden-library:dev
   ```

3. **Access the application:**
   - http://localhost:1430

## Using Build Scripts

### Linux/macOS

```bash
chmod +x scripts/docker-build.sh
./scripts/docker-build.sh
```

### Windows

```cmd
scripts\docker-build.bat
```

## Docker Compose Profiles

### Production Profile (Default)

```bash
docker-compose up -d
```

### Development Profile

```bash
docker-compose --profile dev up -d
```

## Environment Variables

The following environment variables can be configured:

- `NODE_ENV`: Set to `production` or `development`
- `SENTRY_DSN`: Sentry DSN for error tracking
- `ENVIRONMENT`: Environment name for Sentry

## Volumes

- `./logs:/var/log/nginx`: Nginx logs
- `.:/app`: Source code (development only)

## Health Checks

The application includes health checks that monitor:

- Application availability
- Nginx service status

## Troubleshooting

### Common Issues

1. **Port already in use:**

   ```bash
   # Check what's using the port
   netstat -tulpn | grep :8080

   # Use a different port
   docker run -p 8081:80 forbidden-library:latest
   ```

2. **Permission denied:**

   ```bash
   # On Linux/macOS, ensure proper permissions
   sudo chown -R $USER:$USER .
   ```

3. **Build fails:**

   ```bash
   # Clean Docker cache
   docker system prune -a

   # Rebuild without cache
   docker build --no-cache -t forbidden-library:latest .
   ```

### Logs

View application logs:

```bash
# Docker Compose
docker-compose logs -f

# Manual Docker
docker logs <container_id>
```

### Shell Access

Access the container shell:

```bash
# Docker Compose
docker-compose exec forbidden-library sh

# Manual Docker
docker exec -it <container_id> sh
```

## Production Deployment

For production deployment, consider:

1. **Using a reverse proxy** (nginx, traefik)
2. **Setting up SSL/TLS certificates**
3. **Configuring proper logging**
4. **Setting up monitoring and alerting**
5. **Using Docker secrets for sensitive data**

### Example Production Setup

```yaml
version: '3.8'
services:
  forbidden-library:
    image: forbidden-library:latest
    restart: unless-stopped
    environment:
      - NODE_ENV=production
    networks:
      - web
      - internal

  nginx-proxy:
    image: nginx:alpine
    ports:
      - '80:80'
      - '443:443'
    volumes:
      - ./nginx-proxy.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - forbidden-library
    networks:
      - web

networks:
  web:
    external: true
  internal:
    driver: bridge
```

## Security Considerations

1. **Run containers as non-root user**
2. **Use specific image tags instead of `latest`**
3. **Regularly update base images**
4. **Scan images for vulnerabilities**
5. **Use secrets management for sensitive data**

## Performance Optimization

1. **Use multi-stage builds** (already implemented)
2. **Optimize layer caching**
3. **Use .dockerignore** (already configured)
4. **Consider using Alpine Linux** (already used)
5. **Enable gzip compression** (configured in nginx)
