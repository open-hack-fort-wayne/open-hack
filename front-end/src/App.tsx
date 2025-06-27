import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import About from "./pages/About";
import Contact from "./pages/Contact";
import "./App.css";
import AppShell from "./components/AppShell";
import Login from "./pages/Login";
import SignUp from "./pages/SignUp";
import EventsPage from "./pages/events/Index";

function App() {
  return (
    <Router>
      <div className="App">
        <AppShell>
          <main>
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/about" element={<About />} />
              <Route path="/contact" element={<Contact />} />
              <Route path="/login" element={<Login />} />
              <Route path="/signup" element={<SignUp />} />
              <Route path="/events" element={<EventsPage />} />
            </Routes>
          </main>
        </AppShell>
      </div>
    </Router>
  );
}

export default App;
