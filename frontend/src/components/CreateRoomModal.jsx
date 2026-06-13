import React, { useState } from "react";
import ModalWrapper from "./common/ModalWrapper";

const CreateRoomModal = ({ isOpen, onClose, onSuccess }) => {
  const [name, setName] = useState("");
  const [capacity, setCapacity] = useState(20);
  const [desc, setDesc] = useState("");
  const [closeAt, setCloseAt] = useState("");

  const handleSubmit = (e) => {
    e.preventDefault();
    if (!name.trim()) { alert("房间名称不能为空"); return; }
    const payload = {
      name: name.trim(),
      capacity,
      description: desc.trim(),
      isPrivate: false,
    };
    // 如果用户未选择关闭时间，默认设置为365天后（全天开放语义）
    payload.closeAt = closeAt
      ? new Date(closeAt).toISOString()
      : new Date(Date.now() + 86400000 * 365).toISOString();
    onSuccess(payload);
    onClose();
  };

  return (
    <ModalWrapper isOpen={isOpen} onClose={onClose} maxWidth="max-w-md">
      <div className="bg-gradient-to-r from-primary to-secondary p-6 text-white text-center rounded-t-2xl">
        <h3 className="text-xl font-bold mb-2">创建自习室</h3>
        <p className="text-white/80">创建属于你的专属学习空间</p>
      </div>
      <form onSubmit={handleSubmit} className="p-6 space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">房间名称</label>
          <input type="text" value={name} onChange={(e) => setName(e.target.value)} maxLength={20} required placeholder="请输入房间名称"
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">房间容量 (1-50人)</label>
          <input type="number" min={1} max={50} value={capacity} onChange={(e) => setCapacity(parseInt(e.target.value) || 20)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">房间描述（选填）</label>
          <textarea rows={2} maxLength={255} value={desc} onChange={(e) => setDesc(e.target.value)} placeholder="介绍一下你的自习室..."
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none resize-none" />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">关闭时间（选填，不填则全天开放）</label>
          <input type="datetime-local" value={closeAt} onChange={(e) => setCloseAt(e.target.value)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
        </div>
        <div className="flex gap-3 pt-2">
          <button type="button" onClick={onClose} className="flex-1 bg-gray-100 text-gray-600 py-2.5 rounded-lg font-medium hover:bg-gray-200 transition-colors">取消</button>
          <button type="submit" className="flex-1 bg-primary text-white py-2.5 rounded-lg font-medium hover:bg-primary/90 transition-colors shadow-md shadow-primary/20">立即创建</button>
        </div>
      </form>
    </ModalWrapper>
  );
};
export default CreateRoomModal;
