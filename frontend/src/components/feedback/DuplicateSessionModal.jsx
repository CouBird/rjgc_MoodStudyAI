import React from 'react';
import ModalWrapper from '../common/ModalWrapper';

/** 对应 demo.html #duplicate-session-modal */
const DuplicateSessionModal = ({ isOpen, onClose }) => (
    <ModalWrapper isOpen={isOpen} onClose={onClose} maxWidth="max-w-sm">
        <div className="bg-gradient-to-r from-warning to-orange-400 p-6 text-white text-center">
            <div className="w-16 h-16 rounded-full bg-white/20 mx-auto flex items-center justify-center mb-4">
                <i className="fa fa-exclamation-triangle text-3xl"></i>
            </div>
            <h3 className="text-xl font-bold mb-2">无法进行此操作</h3>
        </div>
        <div className="p-6 text-center">
            <p className="text-gray-600 mb-6">
                您当前有正在进行的学习会话，请先结束当前学习再加入其他房间！
            </p>
            <button
                onClick={onClose}
                className="w-full bg-primary text-white py-2.5 rounded-lg hover:bg-primary/90 transition-colors font-medium"
            >
                我知道了
            </button>
        </div>
    </ModalWrapper>
);

export default DuplicateSessionModal;
