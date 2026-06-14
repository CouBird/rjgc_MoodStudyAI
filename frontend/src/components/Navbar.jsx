import React, { useState } from "react";
import { useUserVM } from '../viewmodels';
import { useUser } from '../store/userContext';
import { useStudy } from '../store/studyContext';

export default function Navbar({ currentPage, setCurrentPage, userRole }) {
    const [dropdownOpen, setDropdownOpen] = useState(false);
    const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
    const userVM = useUserVM();

    const getNavLinkClass = (page) => {
        const isActive =
            currentPage === page ||
            (page === "study-rooms" && currentPage === "room-detail");
        return `nav-link text-sm transition-colors ${isActive ? "text-primary font-medium" : "text-gray-600 hover:text-primary"}`;
    };

    const displayName = userVM.displayName;
    const avatarUrl = userVM.avatarUrl;
    const { clearUser } = useUser();
    const study = useStudy();
    const { resetStudy } = study;
    const hasActiveSession = study.sessionStatus === "studying" || study.sessionStatus === "paused";

    const handleLogout = () => {
        resetStudy();
        localStorage.removeItem("token");
        clearUser();
        setCurrentPage("login");
    };

    return (
        <nav className="fixed top-0 left-0 right-0 z-50 bg-white/70 backdrop-blur-[10px] border-b border-gray-200 transition-all duration-300">
            <div className="container mx-auto px-4 py-3 flex justify-between items-center">
                {/* Logo */}
                <div className="flex items-center space-x-2 cursor-pointer" onClick={() => setCurrentPage("home")}>
                    <div className="w-10 h-10 rounded-full bg-gradient-to-r from-primary to-secondary flex items-center justify-center text-white">
                        <i className="fa fa-book text-lg"></i>
                    </div>
                    <h1 className="text-xl font-bold bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
                        心流自习室
                    </h1>
                </div>

                {/* 桌面端导航链接 */}
                <div className="hidden md:flex items-center space-x-8">
                    <button onClick={() => setCurrentPage("home")} className={getNavLinkClass("home")}>首页</button>
                    <button onClick={() => setCurrentPage("study-rooms")} className={getNavLinkClass("study-rooms")}>自习室</button>
                    <button onClick={() => setCurrentPage("statistics")} className={getNavLinkClass("statistics")}>学习统计</button>
                    <button onClick={() => setCurrentPage("profile")} className={getNavLinkClass("profile")}>个人中心</button>
                    {hasActiveSession && (
                        <button onClick={() => setCurrentPage("study-timer")} className="inline-flex items-center bg-primary text-white px-3 py-2 rounded-lg text-sm hover:bg-primary/90 transition-colors">
                            <i className="fa fa-clock-o mr-2"></i>回到计时
                        </button>
                    )}
                </div>

                {/* 头像及下拉菜单 */}
                <div className="flex items-center space-x-4">
                    <div className="relative">
                        <button
                            onClick={() => setDropdownOpen(!dropdownOpen)}
                            className="flex items-center space-x-2 focus:outline-none"
                        >
                            <img src={avatarUrl} alt="用户头像" className="w-8 h-8 rounded-full object-cover border-2 border-primary"  onError={(e) => { e.target.src = "/default-avatar.png"; }} />
                            <span className="hidden md:inline text-sm font-medium text-dark">{displayName}</span>
                            <i className={`fa fa-chevron-down text-xs text-gray-500 transition-transform ${dropdownOpen ? "rotate-180" : ""}`}></i>
                        </button>

                        {dropdownOpen && (
                            <div className="absolute right-0 mt-2 w-48 bg-white rounded-lg shadow-lg py-2 border border-gray-100 animate-fade-in z-50">
                                <button onClick={() => { setCurrentPage("profile"); setDropdownOpen(false); }} className="w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 flex items-center">
                                    <i className="fa fa-user mr-2 text-gray-400"></i>个人中心
                                </button>
                                <button onClick={() => { setCurrentPage("statistics"); setDropdownOpen(false); }} className="w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 flex items-center">
                                    <i className="fa fa-bar-chart mr-2 text-gray-400"></i>学习统计
                                </button>
                                {hasActiveSession && (
                                    <button onClick={() => { setCurrentPage("study-timer"); setDropdownOpen(false); }} className="w-full text-left px-4 py-2 text-sm text-primary hover:bg-gray-100 flex items-center">
                                        <i className="fa fa-clock-o mr-2 text-gray-400"></i>回到计时
                                    </button>
                                )}
                                <div className="border-t border-gray-200 my-1"></div>
                                <button onClick={handleLogout} className="w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-gray-100 flex items-center">
                                    <i className="fa fa-sign-out mr-2"></i>退出登录
                                </button>
                            </div>
                        )}
                    </div>

                    {/* 移动端汉堡菜单按钮 */}
                    <button onClick={() => setMobileMenuOpen(!mobileMenuOpen)} className="md:hidden text-gray-600 focus:outline-none">
                        <i className="fa fa-bars text-xl"></i>
                    </button>
                </div>
            </div>

            {/* 移动端下拉菜单 */}
            {mobileMenuOpen && (
                <div className="md:hidden bg-white border-t border-gray-200 animate-slide-up">
                    <div className="container mx-auto px-4 py-3 space-y-3">
                        <button onClick={() => { setCurrentPage("home"); setMobileMenuOpen(false); }} className={`block w-full text-left py-2 text-sm ${currentPage === "home" ? "text-primary font-medium" : "text-gray-600"}`}>首页</button>
                        <button onClick={() => { setCurrentPage("study-rooms"); setMobileMenuOpen(false); }} className={`block w-full text-left py-2 text-sm ${currentPage === "study-rooms" || currentPage === "room-detail" ? "text-primary font-medium" : "text-gray-600"}`}>自习室</button>
                        <button onClick={() => { setCurrentPage("statistics"); setMobileMenuOpen(false); }} className={`block w-full text-left py-2 text-sm ${currentPage === "statistics" ? "text-primary font-medium" : "text-gray-600"}`}>学习统计</button>
                        <button onClick={() => { setCurrentPage("profile"); setMobileMenuOpen(false); }} className={`block w-full text-left py-2 text-sm ${currentPage === "profile" ? "text-primary font-medium" : "text-gray-600"}`}>个人中心</button>
                        {hasActiveSession && (
                            <button onClick={() => { setCurrentPage("study-timer"); setMobileMenuOpen(false); }} className="block w-full text-left py-2 text-sm text-primary">回到计时</button>
                        )}
                    </div>
                </div>
            )}
        </nav>
    );
}


