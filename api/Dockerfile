# Use official Node.js image
FROM node:18

# Create app directory
WORKDIR /usr/src/app

# Install app dependencies
COPY package*.json ./
RUN npm install

# Bundle app source code
COPY . .

# Expose the app's port
EXPOSE 3000

# Command to run the application
CMD ["npm", "start"]
