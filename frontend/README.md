# Orpheus Frontend

[中文版本](README.zh.md) | English Version

## Overview

This is the frontend application for Orpheus, a modern full-stack application built with Next.js 16, React 19, and TypeScript. The frontend provides a clean, responsive user interface that interacts with the Orpheus backend API for authentication, user management, and data visualization.

## Technology Stack

| Technology | Version | Description |
|------------|---------|-------------|
| Next.js | 16.0.3 | React full-stack framework |
| React | 19.2.0 | UI library |
| TypeScript | 5 | Type-safe JavaScript |
| Tailwind CSS | 4 | Utility-first CSS framework |
| Radix UI | - | Headless UI component library |
| Lucide React | 0.553.0 | Modern icon library |
| Axios | 1.13.2 | HTTP client for API requests |
| pnpm | - | Package manager |

## Features

- **Modern UI/UX**: Clean, responsive design with Tailwind CSS
- **Component Architecture**: Modular, reusable React components
- **Type Safety**: Full TypeScript implementation
- **API Integration**: Seamless communication with Orpheus backend
- **GitHub Integration**: Real-time repository star count display
- **Navigation**: Intuitive navigation with responsive menu
- **Authentication**: User authentication and session management

## Project Structure

```
frontend/
├── app/                    # Next.js App Router directory
│   ├── layout.tsx         # Root layout component
│   ├── page.tsx           # Home page component
│   └── globals.css        # Global styles
├── components/            # React components
│   └── ui/                # UI component library
│       ├── Navbar.tsx     # Navigation bar component
│       ├── main.tsx       # Main content component
│       ├── card.tsx       # Card component
│       ├── button.tsx     # Button component
│       └── alert-dialog.tsx # Alert dialog component
├── lib/                   # Utility library
│   └── utils.ts           # Utility functions
├── public/                # Static assets
│   ├── file.svg           # SVG icons
│   ├── globe.svg          # SVG icons
│   ├── next.svg           # Next.js icon
│   ├── vercel.svg         # Vercel icon
│   └── window.svg         # SVG icons
├── .gitignore             # Git ignore file
├── components.json        # Shadcn/ui configuration
├── eslint.config.mjs      # ESLint configuration
├── next.config.ts         # Next.js configuration
├── package.json           # Dependencies and scripts
├── pnpm-lock.yaml         # pnpm lock file
├── postcss.config.mjs     # PostCSS configuration
└── tsconfig.json          # TypeScript configuration
```

## Getting Started

### Prerequisites

- Node.js 20+
- pnpm package manager

### Installation

1. **Install dependencies**
   ```bash
   pnpm install
   ```

2. **Start development server**
   ```bash
   pnpm dev
   ```

3. **Open your browser**
   Navigate to [http://localhost:3000](http://localhost:3000) to see the application.

### Available Scripts

- `pnpm dev` - Start development server
- `pnpm build` - Build for production
- `pnpm start` - Start production server
- `pnpm lint` - Run ESLint

## Development

### Component Development

Components are organized in the `components/ui/` directory. Each component follows these conventions:

- **TypeScript**: All components are written in TypeScript
- **Props Interface**: Clear prop definitions with TypeScript interfaces
- **Documentation**: JSDoc comments for complex components
- **Styling**: Tailwind CSS for styling
- **Responsiveness**: Mobile-first responsive design

### API Integration

The frontend uses Axios for HTTP requests to the Orpheus backend API:

```typescript
import axios from 'axios';

// Example API call
const response = await axios.get('http://127.0.0.1:8080/api/endpoint');
```

### Styling

- **Tailwind CSS**: Utility-first CSS framework
- **Responsive Design**: Mobile-first approach
- **Component Variants**: Using class-variance-authority for component variants
- **Dark Mode**: Configured for future dark mode support

## Key Components

### Navbar Component

The main navigation component featuring:
- Responsive design with mobile menu
- GitHub repository star count integration
- User authentication links
- Modern styling with hover effects

### Main Component

The primary content container component providing:
- Consistent layout structure
- Responsive content areas
- Integration with other UI components

### UI Components

A collection of reusable UI components:
- **Button**: Customizable button component
- **Card**: Flexible card component
- **AlertDialog**: Modal dialog component

## Configuration

### Environment Variables

Create a `.env.local` file in the root directory:

```env
NEXT_PUBLIC_API_URL=http://127.0.0.1:8080
```

### TypeScript Configuration

The project uses strict TypeScript configuration with:
- Strict type checking
- Path mapping for clean imports
- Next.js optimization

### ESLint Configuration

ESLint is configured with:
- Next.js recommended rules
- TypeScript support
- Consistent code formatting

## Deployment

### Build for Production

```bash
pnpm build
```

### Start Production Server

```bash
pnpm start
```

### Environment-Specific Builds

The application supports different environments:
- Development: `pnpm dev`
- Production: `pnpm build && pnpm start`

## Contributing

1. **Component Creation**: Follow existing component patterns
2. **Type Safety**: Ensure all code is type-safe
3. **Documentation**: Add comments for complex logic
4. **Testing**: Test components across different screen sizes
5. **Code Style**: Follow existing code style and conventions

## Best Practices

- **Component Composition**: Build complex UIs from simple components
- **Props Interface**: Always define clear prop interfaces
- **Error Handling**: Implement proper error handling for API calls
- **Performance**: Use React optimization techniques (memo, callback, etc.)
- **Accessibility**: Ensure components are accessible (ARIA labels, keyboard navigation)

## Troubleshooting

### Common Issues

1. **Port Already in Use**
   ```bash
   # Kill process on port 3000
   lsof -ti:3000 | xargs kill -9
   ```

2. **Dependency Issues**
   ```bash
   # Clear cache and reinstall
   rm -rf node_modules pnpm-lock.yaml
   pnpm install
   ```

3. **TypeScript Errors**
   ```bash
   # Check TypeScript configuration
   pnpm run type-check
   ```

## Support

For issues related to:
- **Frontend**: Create an issue in the repository
- **Backend**: Refer to the backend documentation
- **General**: Check the main project README

---

**Note**: This frontend is part of the Orpheus full-stack application. Please ensure the backend server is running for full functionality.