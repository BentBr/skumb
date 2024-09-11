# Use Node.js as the base image to build the Vue app
FROM node:22-alpine as builder

# Set working directory inside container
WORKDIR /app

# Copy package.json and package-lock.json (if available)
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application files
COPY . .

# Build the app for production
RUN npm run build

# Serve the app using nginx
FROM nginx:alpine

# Copy the built files to the nginx html directory
COPY --from=builder /app/dist /usr/share/nginx/html

# Expose the port where nginx will serve the frontend
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
