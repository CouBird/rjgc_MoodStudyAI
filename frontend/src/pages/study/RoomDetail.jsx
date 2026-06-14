import React, { useState, useEffect } from "react";
import { roomApi } from "../../api/room";
import { toRoomDetailVM } from "../../viewmodels";
import DuplicateSessionModal from "../../components/feedback/DuplicateSessionModal";
import { useStudy } from "../../store/studyContext";
import { resolveAvatarUrl, avatarFallback } from "../../utils";

export default function RoomDetail({ setCurrentPage, isStudying, setIsStudying, setSelectedSeat, setSelectedSeatCode, setActiveRoomId, selectedRoomId }) {
  const study = useStudy();
  const [detailVM, setDetailVM] = useState(null);
  const [selectedId, setSelectedId] = useState(null);
  const [showDup, setShowDup] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [starting, setStarting] = useState(false);

  useEffect(() => {
    if (!selectedRoomId) {
      setError("未指定房间");
      setLoading(false);
      return;
    }
    setLoading(true);
    setError(null);
    roomApi.getRoomDetail(selectedRoomId)
      .then((data) => setDetailVM(toRoomDetailVM(data)))
      .catch((err) => {
        setError(err.message || "加载房间详情失败");
      })
      .finally(() => setLoading(false));
  }, [selectedRoomId]);

  const handleSeatClick = (seat) => {
    if (seat.isOccupied) return;
    setSelectedId(selectedId === seat.seatId ? null : seat.seatId);
  };

  const handleStart = async () => {
    const hasActiveSession = isStudying || study.sessionStatus === "studying" || study.sessionStatus === "paused";
    if (hasActiveSession) {
      setShowDup(true);
      return;
    }
    if (!selectedId) return;
    const chosenSeat = detailVM.seats.find(s => s.seatId === selectedId);
    if (!chosenSeat) return;

    setStarting(true);
    try {
      await study.startStudy({ roomId: selectedRoomId, seatId: chosenSeat.seatId, mode: "normal" });
      setIsStudying(true);
      setActiveRoomId(selectedRoomId);
      setSelectedSeat(chosenSeat.seatId);
      setSelectedSeatCode(chosenSeat.seatCode);
      setCurrentPage("study-timer");
    } catch (err) {
      setError(err.message || "开始学习失败");
    } finally {
      setStarting(false);
    }
  };


  const room = detailVM?.room;
  const seats = detailVM?.seats ?? [];
  const members = detailVM?.members ?? [];
  const occupiedCount = detailVM?.occupiedCount ?? 0;
  const total = selectedId ? occupiedCount + 1 : occupiedCount;
  const selectedSeat = seats.find((s) => s.seatId === selectedId);

  const ownerName = room?.creatorName ?? "未知";
  const roomName = room?.name ?? "自习室";
  const roomDescription = room?.description ?? null;
  const roomCapacity = room?.capacity ?? 25;
  const roomCloseTime = room?.closeTimeDisplay ?? "全天开放";
  const createTime = room?.createdAt
    ? new Date(room.createdAt).toLocaleDateString("zh-CN")
    : null;

  const getSeatClasses = (seat) => {
    if (seat.isOccupied) {
      return "bg-red-100 border-red-500 cursor-not-allowed relative";
    }
    if (selectedId === seat.seatId) {
      return "bg-indigo-200 border-indigo-500 shadow-[0_0_0_2px_rgba(99,102,241,0.5)]";
    }
    return "bg-emerald-100 border-emerald-500 cursor-pointer hover:scale-105";
  };

  if (error) {
    return (
      <div className="container mx-auto px-4 pb-16 min-h-screen">
        <button onClick={() => setCurrentPage("study-rooms")}
          className="flex items-center mb-4 text-gray-600 hover:text-primary transition-colors text-sm">
          <i className="fa fa-arrow-left mr-2"></i>返回自习室列表
        </button>
        <div className="bg-white rounded-xl shadow-md p-12 text-center">
          <div className="text-gray-300 text-5xl mb-4"><i className="fa fa-exclamation-triangle"></i></div>
          <h3 className="text-xl font-semibold text-gray-800 mb-2">加载失败</h3>
          <p className="text-gray-500 mb-6">{error}</p>
          <button onClick={() => window.location.reload()} className="px-6 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors">
            重新加载
          </button>
        </div>
        <DuplicateSessionModal isOpen={showDup} onClose={() => setShowDup(false)} />
    </div>
    );
  }

  return (
    <div className="container mx-auto px-4 pb-16 min-h-screen">
      <div className="py-6">
        <button onClick={() => setCurrentPage("study-rooms")}
          className="flex items-center mb-4 text-gray-600 hover:text-primary transition-colors text-sm">
          <i className="fa fa-arrow-left mr-2"></i>返回自习室列表
        </button>

        {loading ? (
          <div className="bg-white rounded-xl shadow-md p-6 animate-pulse">
            <div className="h-8 bg-gray-200 rounded w-1/3 mb-4"></div>
            <div className="h-4 bg-gray-200 rounded w-2/3 mb-6"></div>
            <div className="grid grid-cols-5 gap-3 mb-6">
              {Array.from({ length: 25 }).map((_, i) => (
                <div key={i} className="h-14 bg-gray-200 rounded-lg"></div>
              ))}
            </div>
          </div>
        ) : (
          <div>
            <div className="bg-white rounded-xl shadow-md overflow-hidden mb-6">
              <div className="h-40 bg-gradient-to-r from-blue-400 to-purple-500 relative">
                <div className="absolute inset-0 bg-black/30"></div>
                <div className="absolute bottom-0 left-0 right-0 p-6 text-white">
                  <div className="flex flex-col md:flex-row md:items-center justify-between">
                    <div>
                      <h2 className="text-2xl font-bold mb-1">{roomName}</h2>
                      {roomDescription && <p className="text-white/80">{roomDescription}</p>}
                    </div>
                    <div className="mt-4 md:mt-0 flex items-center">
                      <span className="bg-white/20 text-white px-3 py-1 rounded-full text-sm mr-3">
                        <i className="fa fa-users mr-1"></i> {occupiedCount}/{roomCapacity}人
                      </span>
                      <span className="bg-success text-white px-3 py-1 rounded-full text-sm">
                        <i className="fa fa-circle mr-1 text-xs"></i> 开放中
                      </span>
                    </div>
                  </div>
                </div>
              </div>
              <div className="p-6">
              <div className="flex flex-col md:flex-row gap-6">


              <div className="md:w-2/3">
                <h3 className="text-lg font-semibold text-gray-800 mb-4">选择座位</h3>
                <div className="bg-gray-50 rounded-lg p-4 mb-6">
                  {seats.length === 0 ? (
                    <div className="text-center py-8 text-gray-400">
                      <i className="fa fa-chair text-3xl mb-2"></i>
                      <p>暂无座位信息</p>
                    </div>
                  ) : (
                    <div className="grid grid-cols-5 gap-3">
                      {seats.map((seat) => {
                        const occupiedName = seat.occupiedBy?.displayName || "用户";
                        const occupiedAvatar = resolveAvatarUrl(seat.occupiedBy?.avatarUrl, occupiedName);

                        return (
                        <button key={seat.seatId}
                          onClick={() => handleSeatClick(seat)}
                          disabled={seat.isOccupied}
                          className={"h-12 md:h-14 rounded-lg border-2 text-xs md:text-sm font-medium transition-all duration-200 flex flex-col items-center justify-center " + getSeatClasses(seat)}>
                          {seat.isOccupied ? (
                            <>
                              <span className="absolute top-0.5 left-1 text-[10px] text-gray-500">{seat.seatCode}</span>
                              <img
                                src={occupiedAvatar}
                                alt={occupiedName}
                                className="w-6 h-6 rounded-full border border-red-200 mt-2 shadow-sm object-cover"
                                onError={(event) => { event.currentTarget.src = avatarFallback(occupiedName); }}
                              />
                            </>
                          ) : (
                            seat.seatCode
                          )}
                        </button>
                        );
                      })}
                    </div>
                  )}
                </div>

                <div className="flex items-center justify-center space-x-6 text-sm text-gray-600 mb-6">
                  <span className="flex items-center">
                    <span className="w-4 h-4 rounded bg-emerald-100 border-2 border-emerald-500 mr-2"></span>可选择
                  </span>
                  <span className="flex items-center">
                    <span className="w-4 h-4 rounded bg-red-100 border-2 border-red-500 mr-2"></span>已占用
                  </span>
                  <span className="flex items-center">
                    <span className="w-4 h-4 rounded bg-indigo-200 border-2 border-indigo-500 mr-2"></span>已选择
                  </span>
                </div>

                <button onClick={handleStart} disabled={!selectedId || starting}
                  className={"w-full px-6 py-3 rounded-lg font-semibold transition-all duration-200 flex items-center justify-center gap-2 " +
                    (selectedId && !starting ? "bg-primary text-white hover:bg-primary/90 shadow-md" : "bg-gray-200 text-gray-400 cursor-not-allowed")}>
                  <i className="fa fa-play mr-2"></i>
                  {starting
                    ? "正在开始学习..."
                    : selectedId
                    ? "锁定 " + (selectedSeat?.seatCode ?? "") + " 号座并开始学习"
                    : "请先选择一个座位"}
                </button>
              </div>

              <div className="md:w-1/3">
                <h3 className="text-lg font-semibold mb-4">房间成员 ({members.length})</h3>
                <div className="bg-gray-50 rounded-lg p-4 max-h-80 overflow-y-auto">
                  {members.length > 0 ? (
                    <div className="space-y-3">
                      {members.map((member, idx) => {
                        const isOwner = member.userId === room?.creator?.userId;
                        const memberName = member.displayName ?? "用户";
                        const memberAvatar = resolveAvatarUrl(member.avatarUrl, memberName);
                        return (
                          <div key={member.userId ?? idx} className="flex items-center p-2 rounded-lg hover:bg-gray-100 transition-colors">
                            <img
                              src={memberAvatar}
                              alt={memberName}
                              className="w-8 h-8 rounded-full mr-3 object-cover"
                              onError={(event) => { event.currentTarget.src = avatarFallback(memberName); }}
                            />
                            <span className="text-sm flex-1 truncate">{memberName}</span>
                            {isOwner && <span className="bg-primary/10 text-primary text-xs px-2 py-1 rounded-full ml-auto">房主</span>}
                          </div>
                        );
                      })}
                    </div>
                  ) : (
                    <div className="text-center text-gray-400 py-8">
                      <i className="fa fa-users text-3xl mb-2"></i>
                      <p className="text-sm">暂无成员信息</p>
                    </div>
                  )}
                </div>

                <div className="mt-6">
                  <h4 className="font-medium mb-2">房间信息</h4>
                  <div className="bg-gray-50 rounded-lg p-4 text-sm text-gray-600 space-y-2">
                    <div className="flex justify-between">
                      <span>房主</span>
                      <span className="font-medium">{ownerName}</span>
                    </div>
                    {createTime && (
                      <div className="flex justify-between">
                        <span>创建时间</span>
                        <span>{createTime}</span>
                      </div>
                    )}
                    <div className="flex justify-between">
                      <span>开放时长</span>
                      <span>{roomCloseTime}</span>
                    </div>
                    {roomDescription && (
                      <div className="pt-2 border-t border-gray-200">
                        <span className="text-gray-500">简介：</span>
                        <p className="mt-1 text-gray-600">{roomDescription}</p>
                      </div>
                    )}
                  </div>
              </div>
              </div>
            </div>
            </div>
          </div>
          </div>
        )}
      </div>
      <DuplicateSessionModal isOpen={showDup} onClose={() => setShowDup(false)} />
    </div>
  );
}

