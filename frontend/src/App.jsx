import React, { useState, lazy, Suspense } from "react";
import Navbar from "./components/Navbar";
import { StudyProvider } from "./store/studyContext";
import { UserProvider } from "./store/userContext";

const HomePage = lazy(() => import("./pages/home/index"));
const LoginPage = lazy(() => import("./pages/login/index"));
const RoomPage = lazy(() => import("./pages/room/index"));
const HallList = lazy(() => import("./pages/study/HallList"));
const RoomDetail = lazy(() => import("./pages/study/RoomDetail"));
const StudyTimer = lazy(() => import("./pages/study/StudyTimer"));
const Dashboard = lazy(() => import("./pages/dashboard/index"));
const EmotionPage = lazy(() => import("./pages/emotion/index"));
const ProfilePage = lazy(() => import("./pages/profile/index"));
const AdminPage = lazy(() => import("./pages/admin/index"));

const LoadingFallback = () => (
  <div className="min-h-screen flex items-center justify-center">
    <div className="animate-spin w-10 h-10 border-4 border-primary border-t-transparent rounded-full"></div>
  </div>
);

function decodeJwtRole(token) {
  if (!token) return null;
  try {
    const payload = token.split(".")[1];
    if (!payload) return null;
    const base64 = payload.replace(/-/g, "+").replace(/_/g, "/");
    const padded = base64 + "=".repeat((4 - (base64.length % 4)) % 4);
    const decoded = atob(padded);
    const claims = JSON.parse(decoded);
    return claims?.role || null;
  } catch {
    return null;
  }
}

function readInitialAuth() {
  if (typeof window === "undefined") {
    return { page: "login", role: "user" };
  }

  const token = localStorage.getItem("token");
  const storedRole = localStorage.getItem("role");
  const role = decodeJwtRole(token) || storedRole || "user";

  if (token && role === "admin") {
    return { page: "admin", role: "admin" };
  }

  if (token) {
    return { page: "home", role: "user" };
  }

  return { page: "login", role: "user" };
}

function App() {
  const initialAuth = readInitialAuth();
  const [currentPage, setCurrentPage] = useState(initialAuth.page);
  const [userRole, setUserRole] = useState(initialAuth.role);
  const [isStudying, setIsStudying] = useState(false);
  const [selectedSeat, setSelectedSeat] = useState(null);
  const [selectedSeatCode, setSelectedSeatCode] = useState(null);
  const [activeRoomId, setActiveRoomId] = useState(null);
  const [selectedRoomId, setSelectedRoomId] = useState(null);
  const hideNavbar = currentPage === "login" || currentPage === "study-timer" || currentPage === "admin";

  const handleUserReady = (user) => {
    if (user?.role) {
      localStorage.setItem("role", user.role);
      setUserRole(user.role);
    }

    if (currentPage === "login") {
      setCurrentPage(user?.role === "admin" ? "admin" : "home");
    }
  };

  const renderPage = () => {
    const p = { currentPage, setCurrentPage, isStudying, setIsStudying, selectedSeat, setSelectedSeat, selectedSeatCode, setSelectedSeatCode, activeRoomId, setActiveRoomId, userRole, setUserRole, selectedRoomId, setSelectedRoomId };
    switch (currentPage) {
      case "login": return <LoginPage {...p} />;
      case "home": return <HomePage {...p} />;
      case "study-rooms": return <HallList {...p} />;
      case "room-detail": return <RoomDetail {...p} />;
      case "study-timer": return <StudyTimer {...p} />;
      case "statistics": return <Dashboard {...p} />;
      case "emotion": return <EmotionPage {...p} />;
      case "profile": return <ProfilePage {...p} />;
      case "admin": return <AdminPage {...p} />;
      default: return <LoginPage {...p} />;
    }
  };

  return (
    <UserProvider onUserReady={handleUserReady} setUserRole={setUserRole}>
      <StudyProvider>
        <div className="min-h-screen bg-gradient-to-br from-calm via-light to-purple-50 text-dark font-sans antialiased">
          {!hideNavbar && <Navbar currentPage={currentPage} setCurrentPage={setCurrentPage} userRole={userRole} />}
          <main className={currentPage === "study-timer" || currentPage === "admin" ? "" : "pt-24"}>
            <Suspense fallback={<LoadingFallback />}>{renderPage()}</Suspense>
          </main>
        </div>
      </StudyProvider>
    </UserProvider>
  );
}
export default App;