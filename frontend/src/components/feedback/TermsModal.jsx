import React from "react";
import ModalWrapper from "../common/ModalWrapper";

const TermsModal = ({ isOpen, onClose }) => (
  <ModalWrapper isOpen={isOpen} onClose={onClose} maxWidth="max-w-lg">
    <div className="bg-gradient-to-r from-primary to-secondary p-6 text-white text-center rounded-t-2xl">
      <h3 className="text-xl font-bold">用户协议</h3>
    </div>
    <div className="p-6 overflow-y-auto max-h-96 space-y-4 text-sm text-gray-700">
      <h4 className="font-bold text-base">1. 服务条款</h4>
      <p>欢迎使用AI情绪治愈自习打卡空间（以下简称"本服务"）。使用本服务即表示您同意本协议全部条款。</p>
      <h4 className="font-bold text-base">2. 账号注册</h4>
      <p>用户须使用真实手机号注册账号，密码长度不少于8位且须同时包含字母与数字。</p>
      <h4 className="font-bold text-base">3. 用户行为规范</h4>
      <p>用户在使用本服务过程中，应遵守法律法规，不得发布违法违规信息。</p>
      <h4 className="font-bold text-base">4. 服务内容</h4>
      <p>本服务提供自习室创建与加入、学习计时、情绪记录、AI辅助反馈等功能。</p>
      <h4 className="font-bold text-base">5. 免责声明</h4>
      <p>本服务提供的AI情绪反馈仅供参考，不构成医疗或心理咨询建议。</p>
    </div>
    <div className="p-4 border-t border-gray-100">
      <button onClick={onClose} className="w-full bg-primary text-white py-2.5 rounded-lg font-medium hover:bg-primary/90 transition-colors">我知道了</button>
    </div>
  </ModalWrapper>
);
export default TermsModal;
