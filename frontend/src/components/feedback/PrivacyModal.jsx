import React from "react";
import ModalWrapper from "../common/ModalWrapper";

const PrivacyModal = ({ isOpen, onClose }) => (
  <ModalWrapper isOpen={isOpen} onClose={onClose} maxWidth="max-w-lg">
    <div className="bg-gradient-to-r from-primary to-secondary p-6 text-white text-center rounded-t-2xl">
      <h3 className="text-xl font-bold">隐私政策</h3>
    </div>
    <div className="p-6 overflow-y-auto max-h-96 space-y-4 text-sm text-gray-700">
      <h4 className="font-bold text-base">1. 信息收集</h4>
      <p>我们收集的信息包括：注册时提供的手机号、昵称、头像；学习记录、打卡数据、情绪标签等。</p>
      <h4 className="font-bold text-base">2. 信息使用</h4>
      <p>我们使用您的信息用于：提供和维护服务；生成个人学习统计与情绪趋势分析。</p>
      <h4 className="font-bold text-base">3. 信息保护</h4>
      <p>我们采用加密技术保护您的个人信息，密码经加盐哈希处理后存储。</p>
    </div>
    <div className="p-4 border-t border-gray-100">
      <button onClick={onClose} className="w-full bg-primary text-white py-2.5 rounded-lg font-medium hover:bg-primary/90 transition-colors">我知道了</button>
    </div>
  </ModalWrapper>
);
export default PrivacyModal;
