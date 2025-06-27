# Fort Wayne Open Hack

A modern web application built for the Fort Wayne Open Hack, featuring a clean and responsive design with React, Vite, Tailwind CSS, and Shadcn/ui components.

## 🚀 Tech Stack

- **Frontend Framework**: React 19.1.0
- **Build Tool**: Vite 7.0.0
- **Styling**: Tailwind CSS 4.1.11
- **UI Components**: Shadcn/ui (New York style)
- **Routing**: React Router DOM 7.6.2
- **Icons**: Lucide React
- **Language**: TypeScript
- **Linting**: ESLint

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

- Node.js (version 18 or higher)
- npm or yarn package manager

## 🛠️ Installation & Setup

1. **Clone the repository**

   ```bash
   git clone <repository-url>
   cd front-end
   ```

2. **Install dependencies**

   ```bash
   npm install
   ```

3. **Start the development server**

   ```bash
   npm run dev
   ```

4. **Open your browser**
   Navigate to `http://localhost:5173` to view the application

## 📁 Project Structure

```
front-end/
├── public/                 # Static assets
├── src/
│   ├── components/         # Reusable UI components
│   │   ├── ui/            # Shadcn/ui components
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   ├── input.tsx
│   │   │   └── navigation-menu.tsx
│   │   ├── AppShell.tsx   # Main app layout wrapper
│   │   ├── Events.tsx     # Events component
│   │   └── Navigation.tsx # Main navigation component
│   ├── pages/             # Page components
│   │   ├── Home.tsx       # Home page
│   │   ├── About.tsx      # About page
│   │   ├── Contact.tsx    # Contact page
│   │   └── Login.tsx      # Login page
│   ├── lib/               # Utility functions and configurations
│   ├── assets/            # Images, fonts, and other assets
│   ├── App.tsx            # Main App component with routing
│   ├── main.tsx           # Application entry point
│   └── index.css          # Global styles and Tailwind imports
├── components.json        # Shadcn/ui configuration
├── vite.config.ts         # Vite configuration
├── tsconfig.json          # TypeScript configuration
└── package.json           # Dependencies and scripts
```

## 🎯 Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint

## 🎨 Features

- **Responsive Design**: Mobile-first approach with Tailwind CSS
- **Modern UI**: Clean and accessible components using Shadcn/ui
- **Type Safety**: Full TypeScript support
- **Routing**: Client-side routing with React Router
- **Component Library**: Reusable UI components with consistent styling
- **Fast Development**: Hot module replacement with Vite

## 🧩 Components

### UI Components (Shadcn/ui)

- **Button**: Versatile button component with multiple variants
- **Card**: Container component for content organization
- **Input**: Form input component with proper styling
- **Navigation Menu**: Accessible navigation menu component

### Custom Components

- **AppShell**: Main layout wrapper providing consistent structure
- **Navigation**: Main navigation bar with routing
- **Events**: Component for displaying event information

## 📱 Pages

- **Home** (`/`) - Landing page with main content
- **About** (`/about`) - Information about the project/event
- **Contact** (`/contact`) - Contact form and information
- **Login** (`/login`) - User authentication page

## 🎨 Styling

The project uses Tailwind CSS with a custom configuration:

- **Base Color**: Slate
- **CSS Variables**: Enabled for theme customization
- **Style**: New York (Shadcn/ui style)
- **Icons**: Lucide React icon library

## 🔧 Configuration Files

- `components.json` - Shadcn/ui component configuration
- `vite.config.ts` - Vite build and development settings
- `tsconfig.json` - TypeScript compiler options
- `eslint.config.js` - ESLint rules and configuration

## 🚀 Deployment

To build the application for production:

```bash
npm run build
```

The built files will be in the `dist/` directory, ready for deployment to any static hosting service.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is part of the Fort Wayne Open Hack event.

## 🆘 Support

For support and questions, please reach out to the project maintainers or create an issue in the repository.
