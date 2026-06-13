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

function App() {
  const [currentPage, setCurrentPage] = useState("login");
  const [userRole, setUserRole] = useState("user");
  const [isStudying, setIsStudying] = useState(false);
  const [selectedSeat, setSelectedSeat] = useState(null);
  const [selectedSeatCode, setSelectedSeatCode] = useState(null);
  const [activeRoomId, setActiveRoomId] = useState(null);
  const [selectedRoomId, setSelectedRoomId] = useState(null);
  const hideNavbar = currentPage === "login" || currentPage === "study-timer" || currentPage === "admin";

  // Token 恢复时自动跳转首页
  const handleUserReady = () => {
    if (currentPage === "login") {
      setCurrentPage("home");
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


