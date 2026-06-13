/**
 * RoomVM — 统一自习室视图模型
 */

import { toUserVM } from "./_userVM";

// 房间卡片渐变色
const GRADIENTS = [
  "from-blue-400 to-purple-500",
  "from-green-400 to-teal-500",
  "from-orange-400 to-red-500",
  "from-indigo-400 to-blue-500",
  "from-pink-400 to-rose-500",
  "from-cyan-400 to-blue-500",
  "from-yellow-400 to-orange-500",
  "from-purple-400 to-pink-500",
];

/**
 * 格式化关闭时间
 */
function formatCloseTime(dateStr) {
  if (!dateStr) return null;
  try {
    const d = new Date(dateStr);
    const h = String(d.getHours()).padStart(2, "0");
    const m = String(d.getMinutes()).padStart(2, "0");
    return `${h}:${m}`;
  } catch {
    return null;
  }
}

/**
 * 将单个 RoomResponse 转为 RoomVM
 */
export function toRoomVM(room, index) {
  if (!room) return null;

  const id = room.roomId ?? room.id ?? index;
  const gradientClass = GRADIENTS[(id % GRADIENTS.length)];

  // 创建者信息 —— 唯一来源：creator 对象
  const creator = room.creator
    ? toUserVM({
        userId: room.creator.userId,
        nickname: room.creator.nickname,
        avatarUrl: room.creator.avatarUrl,
      })
    : null;

  return Object.freeze({
    id,
    name: room.name ?? null,
    description: room.description ?? null,
    status: room.status ?? "open",
    isOpen: room.status === "open",
    capacity: room.capacity ?? 25,
    currentMembers: room.currentMembers ?? 0,
    isPrivate: room.isPrivate ?? false,
    // 创建者（唯一标识）
    creator,
    creatorName: creator?.displayName ?? null,
    creatorAvatar: creator?.avatarUrl ?? 'unknown-avatar',
  
    createdAt: room.createdAt ?? null,
    openAt: room.openAt ?? null,
    closeAt: room.closeAt ?? null,
    closeTimeDisplay: formatCloseTime(room.closeAt),
    
    gradientClass,
  });
}

/**
 * 将 SeatResponse 转为 SeatVM
 */
export function toSeatVM(seat, index) {
  if (!seat) return null;
  return Object.freeze({
    seatId: seat.seatId ?? String(index + 1),
    seatCode: seat.seatCode ?? null,
    status: seat.status ?? "available",
    isOccupied: seat.status === "occupied",
    occupiedBy: seat.occupiedBy
      ? toUserVM({
          userId: seat.occupiedBy.userId,
          nickname: seat.occupiedBy.nickname,
          avatarUrl: seat.occupiedBy.avatarUrl,
        })
      : null,
  });
}


export function toRoomDetailVM(data) {
  if (!data) return null;

  const room = data.room ? toRoomVM(data.room) : null;
  const seats = Array.isArray(data.seats)
    ? data.seats.map((s, i) => toSeatVM(s, i)).filter(Boolean)
    : [];
  const members = Array.isArray(data.members)
    ? data.members.map((m) => toUserVM({
        userId: m.userId ?? m.id,
        nickname: m.nickname,
        avatarUrl: m.avatarUrl,
      })).filter(Boolean)
    : [];

  return Object.freeze({
    room,
    seats,
    members,
    seatCount: seats.length,
    memberCount: members.length,
    occupiedCount: seats.filter((s) => s.isOccupied).length,
  });
}

/**
 * 将 RoomListResponse 转为 RoomVM 数组
 */
export function toRoomListVM(data) {
  const rawList = Array.isArray(data)
    ? data
    : (data?.items ?? data?.list ?? []);
  return rawList.map((r, i) => toRoomVM(r, i)).filter(Boolean);
}

export { GRADIENTS };


