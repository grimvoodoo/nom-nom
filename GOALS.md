# Project Goals

## Vision
nom-nom is a web-based cookbook platform that helps users manage recipes based on ingredients they actually have. By maintaining a personal "larder" (ingredient inventory), users can discover what they can cook right now without a trip to the store.

## Core Features

### Recipe Management
- [ ] Browse and search recipes
- [ ] Add, edit, and delete recipes
- [ ] Categorize recipes (cuisine, meal type, dietary restrictions)
- [ ] Recipe ratings and notes

### Larder (Ingredient Inventory)
- [ ] Track ingredients users have on hand
- [ ] Set quantities and expiration dates
- [ ] Organize by category (pantry, fridge, freezer)
- [ ] Shopping list generation for missing ingredients

### Smart Recipe Filtering
- [ ] Filter recipes by available larder ingredients
- [ ] "What can I make?" discovery mode
- [ ] Highlight recipes missing only 1-2 ingredients
- [ ] Suggest ingredient substitutions

### AI Ingredient Recognition (Microservice)
- [ ] Upload photos of ingredients to auto-populate larder
- [ ] Scan shopping receipts to update inventory
- [ ] OCR for handwritten shopping lists
- [ ] Ingredient identification from pantry/fridge photos

## Architecture

### Planned Services
| Service | Technology | Purpose |
|---------|------------|---------|
| Web Backend | Rust | Core API, recipe/larder management |
| Frontend | TBD | User interface |
| AI Service | TBD | Image processing, ingredient extraction |
| Database | TBD | Data persistence |

## Milestones

### Phase 1: Foundation
- [ ] Basic recipe CRUD API
- [ ] Database schema design
- [ ] User authentication

### Phase 2: Larder System
- [ ] Ingredient tracking API
- [ ] Recipe-to-larder matching logic
- [ ] Basic filtering functionality

### Phase 3: AI Integration
- [ ] Image upload endpoint
- [ ] AI service for ingredient recognition
- [ ] Integration with larder updates

### Phase 4: Polish
- [ ] Frontend development
- [ ] Mobile responsiveness
- [ ] Performance optimization
