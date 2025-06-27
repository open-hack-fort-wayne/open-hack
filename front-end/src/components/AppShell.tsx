import { Outlet } from "react-router-dom";
import Navigation from "./Navigation";

function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex flex-col min-h-screen">
      <header className="z-50 sticky top-0 bg-white">
        <Navigation />
      </header>
      <main className="flex-grow">
        {/* <Outlet /> */}
        {children}
      </main>
    </div>
  );
}

export default AppShell;
