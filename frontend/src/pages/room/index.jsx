import React, { useState, useEffect } from "react";
import { roomApi } from "../../api/room";
import { toRoomListVM } from "../../viewmodels";
import DuplicateSessionModal from "../../components/feedback/DuplicateSessionModal";
import CreateRoomModal from "../../components/CreateRoomModal";

export default function RoomPage({ setCurrentPage, isStudying, setSelectedRoomId }) {
  const [query, setQuery] = useState("");
  const [showDup, setShowDup] = useState(false);
  const [showCreate, setShowCreate] = useState(false);
  const [rooms, setRooms] = useState([]);
  const [loading, setLoading] = useState(true);
  const [createLoading, setCreateLoading] = useState(false);

  useEffect(() => {
    roomApi.getRoomList()
      .then((data) => setRooms(toRoomListVM(data)))
      .catch(() => setRooms([]))
      .finally(() => setLoading(false));
  }, []);

  const filtered = rooms.filter((r) =>
    r.name && r.name.toLowerCase().includes(query.toLowerCase())
  );

  const handleRoomCreated = async (payload) => {
    setCreateLoading(true);
    try {
      await roomApi.createRoom(payload);
      setShowCreate(false);
      // Refresh room list after creation
      const data = await roomApi.getRoomList();
      setRooms(toRoomListVM(data));
    } catch (err) {
      alert(err?.message || "创建自习室失败");
    } finally {
      setCreateLoading(false);
    }
  };

  return (
    <div className="container mx-auto px-4 pb-16 min-h-screen">
      <div className="mb-6">
        <div className="flex flex-col md:flex-row md:items-center justify-between mb-6">
          <h2 className="text-2xl font-bold text-gray-800 mb-4 md:mb-0">自习室大厅</h2>
          <div className="flex flex-col sm:flex-row gap-3">
            <div className="relative">
              <input type="text" placeholder="搜索自习室..." value={query} onChange={(e) => setQuery(e.target.value)}
                className="pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary transition-colors w-full sm:w-64 outline-none" />
              <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-500"><i className="fa fa-search"></i></span>
            </div>
            <button onClick={() => setShowCreate(true)} disabled={createLoading} className="bg-primary text-white px-4 py-2 rounded-lg hover:bg-primary/90 transition-colors flex items-center justify-center disabled:opacity-60">
              <i className="fa fa-plus mr-2"></i> {createLoading ? "创建中..." : "创建自习室"}
            </button>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filtered.map((room) => (
          <div key={room.id} onClick={() => {
            if (isStudying) { setShowDup(true); return; }
            if (!room.isOpen) return;
            setSelectedRoomId(room.id); setCurrentPage("room-detail");
          }}
            className={"bg-white rounded-xl shadow-md overflow-hidden card-hover " + (room.isOpen ? "cursor-pointer" : "opacity-75 cursor-not-allowed")}>
            <div className={"h-32 bg-gradient-to-r " + room.gradientClass + " relative"}>
              <div className="absolute inset-0 bg-black/20"></div>
              <span className="absolute top-3 right-3 bg-white/20 text-white text-xs px-2 py-1 rounded-full backdrop-blur-sm">
                {room.currentMembers}/{room.capacity}人
              </span>
              <div className="absolute bottom-3 left-3 text-white">
                <h3 className="font-bold text-lg">{room.name}</h3>
                <p className="text-sm text-white/80">{room.description}</p>
              </div>
            </div>
            <div className="p-4">
              <div className="flex items-center mb-3">
                <img src={"https://picsum.photos/id/" + room.creatorAvatar + "/100/100"} alt="房主" className="w-8 h-8 rounded-full mr-2" />
                <span className="text-sm text-gray-600">房主：{room.creatorName}</span>
              </div>
              <p className="text-sm text-gray-600 mb-4 line-clamp-2">{room.description}</p>
              <div className="flex justify-between items-center">
                <span className="text-xs text-gray-500"><i className="fa fa-clock-o mr-1"></i> {room.closeTimeDisplay}</span>
                {room.isOpen ? (
                  <button className="bg-primary text-white px-4 py-1 rounded-lg text-sm hover:bg-primary/90 transition-colors">加入</button>
                ) : (
                  <span className="bg-gray-300 text-white px-4 py-1 rounded-lg text-sm">已关闭</span>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>
      {filtered.length === 0 && !loading && (
        <div className="text-center py-20 bg-white rounded-xl shadow-sm border border-gray-100">
          <div className="text-gray-300 text-5xl mb-3"><i className="fa fa-folder-open-o"></i></div>
          <p className="text-gray-500">没有找到符合条件的自习室</p>
        </div>
      )}
      <DuplicateSessionModal isOpen={showDup} onClose={() => setShowDup(false)} />
      <CreateRoomModal isOpen={showCreate} onClose={() => setShowCreate(false)} onSuccess={handleRoomCreated} />
    </div>
  );
}
