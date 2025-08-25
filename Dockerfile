# Multi-stage Dockerfile for Forbidden Library
# Build stage for the SvelteKit frontend
FROM node:20-alpine AS frontend-builder

# Set working directory
WORKDIR /app

# Install pnpm
RUN npm install -g pnpm

# Copy package files
COPY package.json pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy source code
COPY . .

# Build the frontend
RUN pnpm run build

# Production stage
FROM nginx:alpine AS production

# Copy built frontend from builder stage
COPY --from=frontend-builder /app/build /usr/share/nginx/html

# Copy custom nginx configuration
COPY docker/nginx.conf /etc/nginx/nginx.conf

# Expose port 80
EXPOSE 80

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost/ || exit 1

# Start nginx
CMD ["nginx", "-g", "daemon off;"]