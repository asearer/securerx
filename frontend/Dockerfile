# Stage 1: Build React/Vite app
FROM node:20-alpine AS build
WORKDIR /app

# Copy and install dependencies
COPY package.json ./
RUN npm install --silent

# Copy source code
COPY . .

# Build production app
RUN npm run build

# Stage 2: Serve with Nginx
FROM nginx:alpine
WORKDIR /usr/share/nginx/html

# Remove default nginx content
RUN rm -rf ./*

# Copy build from previous stage
COPY --from=build /app/dist .

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 80
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
