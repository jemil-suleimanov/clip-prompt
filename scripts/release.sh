#!/bin/bash

# Release script for ClipPrompt
# Usage: ./scripts/release.sh [patch|minor|major]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version type is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Please specify version type (patch, minor, or major)${NC}"
    echo "Usage: $0 [patch|minor|major]"
    exit 1
fi

VERSION_TYPE=$1

# Validate version type
if [[ ! "$VERSION_TYPE" =~ ^(patch|minor|major)$ ]]; then
    echo -e "${RED}Error: Version type must be patch, minor, or major${NC}"
    exit 1
fi

echo -e "${YELLOW}🚀 Starting release process for ClipPrompt...${NC}"

# Get current version from package.json
CURRENT_VERSION=$(node -p "require('./package.json').version")
echo -e "${GREEN}Current version: $CURRENT_VERSION${NC}"

# Bump version using npm
echo -e "${YELLOW}Bumping version ($VERSION_TYPE)...${NC}"
npm version $VERSION_TYPE --no-git-tag-version

# Get new version
NEW_VERSION=$(node -p "require('./package.json').version")
echo -e "${GREEN}New version: $NEW_VERSION${NC}"

# Update Tauri config version
echo -e "${YELLOW}Updating Tauri configuration...${NC}"
node -e "
const fs = require('fs');
const config = JSON.parse(fs.readFileSync('./src-tauri/tauri.conf.json', 'utf8'));
config.version = '$NEW_VERSION';
fs.writeFileSync('./src-tauri/tauri.conf.json', JSON.stringify(config, null, 2));
"

# Stage changes
echo -e "${YELLOW}Staging changes...${NC}"
git add package.json src-tauri/tauri.conf.json

# Commit changes
echo -e "${YELLOW}Committing version bump...${NC}"
git commit -m "chore: bump version to $NEW_VERSION"

# Create and push tag
echo -e "${YELLOW}Creating git tag...${NC}"
git tag "v$NEW_VERSION"

echo -e "${YELLOW}Pushing changes and tag...${NC}"
git push origin main
git push origin "v$NEW_VERSION"

echo -e "${GREEN}✅ Release process completed!${NC}"
echo -e "${GREEN}📦 GitHub Actions will now build and create release v$NEW_VERSION${NC}"
echo -e "${YELLOW}⏳ Check the Actions tab in your GitHub repository for build progress${NC}" 