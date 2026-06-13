import React, { useState, useEffect } from "react";
import { roomApi } from "../../api/room";
import { statisticsApi } from "../../api/statistics";
import { useUserVM } from "../../viewmodels";
import { toRoomVM, toTodayStatsVM } from "../../viewmodels";
import CreateRoomModal from "../../components/CreateRoomModal";

export default function HomePage({ setCurrentPage, setSelectedRoomId }) {
  const userVM = useUserVM();
  const [rooms, setRooms] = useState([]);
  const [showCreate, setShowCreate] = useState(false);
  const [todayStatsVM, setTodayStatsVM] = useState(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [roomData, statsData] = await Promise.all([
          roomApi.getRoomList({ status: "open", page: 1, pageSize: 3 }).catch(() => ({ items: [] })),
          statisticsApi.getTodayStats().catch(() => null),
        ]);
        const rawList = Array.isArray(roomData) ? roomData : (roomData?.items || roomData?.list || roomData?.rooms || []);
        setRooms(rawList.map((r, i) => toRoomVM(r, i)).filter((r) => r.isOpen).slice(0, 3));
        if (statsData) {
          setTodayStatsVM(toTodayStatsVM(statsData));
        }
      } catch (err) {
        console.error("Failed to load:", err);
      }
    };
    fetchData();
  }, []);

  const getTagClass = (tag) => {
    if (tag === "热门") return "text-xs px-2 py-1 rounded-full bg-success/10 text-success";
    if (tag === "推荐") return "text-xs px-2 py-1 rounded-full bg-primary/10 text-primary";
    return "text-xs px-2 py-1 rounded-full bg-gray-100 text-gray-600";
  };

  const displayName = userVM.displayName ?? "访客";
  const { todayHours: studyHours = "0", streakDays = 0, latestEmotion: todayEmotion = "" } = todayStatsVM ?? {};

  return (
    <div className="container mx-auto px-4 pb-16 min-h-screen">
      <div className="mb-8">
        <div className="bg-gradient-to-r from-primary/10 to-secondary/10 rounded-2xl p-6 md:p-8">
          <div className="flex flex-col md:flex-row md:items-center justify-between">
            <div className="flex-1">
              <h2 className="text-2xl md:text-3xl font-bold mb-2 text-dark">你好，{displayName} <span className="animate-bounce inline-block">👋</span></h2>
              <p className="text-gray-600 mb-4">今天也要加油学习喔！保持良好的学习心态</p>
              <div className="flex flex-wrap gap-3">
                <div className="bg-white rounded-lg px-4 py-2 shadow-sm">
                  <span className="text-sm text-gray-500">今日学习</span>
                  <p className="text-xl font-bold text-primary">{studyHours}小时</p>
                </div>
                <div className="bg-white rounded-lg px-4 py-2 shadow-sm">
                  <span className="text-sm text-gray-500">连续打卡</span>
                  <p className="text-xl font-bold text-emerald-500">{streakDays}天</p>
                </div>
                <div className="bg-white rounded-lg px-4 py-2 shadow-sm">
                  <span className="text-sm text-gray-500">今日心情</span>
                  <p className="text-xl font-bold text-warning">{todayEmotion}</p>
                </div>
              </div>
            </div>
            <div className="flex-shrink-0">
              <img src="https://picsum.photos/id/366/300/200" alt="学习插图" className="w-64 h-auto rounded-lg shadow-md object-cover" />
            </div>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div className="bg-white rounded-xl shadow-md p-6 card-hover">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-gray-800">快速开始</h3>
            <div className="w-10 h-10 rounded-full bg-primary/10 flex items-center justify-center text-primary"><i className="fa fa-play"></i></div>
          </div>
          <p className="text-gray-500 text-sm mb-4">立即进入自习室开始学习，专注当下，提升效率</p>
          <button onClick={() => setCurrentPage("study-rooms")} className="w-full bg-primary text-white py-2 rounded-lg hover:bg-primary/90 transition-colors">进入自习室</button>
        </div>
        <div className="bg-white rounded-xl shadow-md p-6 card-hover">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-gray-800">创建房间</h3>
            <div className="w-10 h-10 rounded-full bg-secondary/10 flex items-center justify-center text-secondary"><i className="fa fa-plus"></i></div>
          </div>
          <p className="text-gray-500 text-sm mb-4">创建自己的专属自习室，邀请好友一起学习</p>
          <button onClick={() => setShowCreate(true)} className="w-full bg-secondary text-white py-2 rounded-lg hover:bg-secondary/90 transition-colors">创建自习室</button>
        </div>
      </div>

      <div className="bg-white rounded-xl shadow-sm p-6">
        <div className="flex items-center justify-between mb-6">
          <h3 className="text-lg font-semibold text-gray-800">推荐自习室</h3>
          <button onClick={() => setCurrentPage("study-rooms")} className="text-primary text-sm hover:underline">查看全部</button>
        </div>
        <div className="space-y-4">
          {rooms.map((room) => (
            <div key={room.id} onClick={() => { setSelectedRoomId(room.id); setCurrentPage("room-detail"); }}
              className="border border-gray-200 rounded-lg p-4 hover:border-primary/50 transition-colors cursor-pointer">
              <div className="flex justify-between items-start">
                <div>
                  <h4 className="font-medium text-gray-800 mb-1">{room.name}</h4>
                  <p className="text-sm text-gray-500 mb-2">{room.description}</p>
                  <div className="flex items-center text-xs text-gray-400 space-x-4">
                    <span><i className="fa fa-users mr-1.5"></i> {room.currentMembers}/{room.capacity}人</span>
                    <span className="text-emerald-500"><i className="fa fa-clock-o mr-1.5"></i> {room.isOpen ? "开放中" : "已关闭"}</span>
                  </div>
                </div>
                <span className={getTagClass("推荐")}>推荐</span>
              </div>
            </div>
          ))}
        </div>
      </div>
      <CreateRoomModal isOpen={showCreate} onClose={() => setShowCreate(false)} onSuccess={() => setCurrentPage("study-rooms")} />
    </div>
  );
}
