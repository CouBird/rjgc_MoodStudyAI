import React from 'react';
import ModalWrapper from '../common/ModalWrapper';


const DisabledAccountModal = ({ isOpen, onClose }) => (
    <ModalWrapper isOpen={isOpen} onClose={onClose} maxWidth="max-w-sm">
        <div className="bg-gradient-to-r from-danger to-red-400 p-6 text-white text-center">
            <div className="w-16 h-16 rounded-full bg-white/20 mx-auto flex items-center justify-center mb-4">
                <i className="fa fa-ban text-3xl"></i>
            </div>
            <h3 className="text-xl font-bold mb-2">账号已被禁用</h3>
        </div>
        <div className="p-6 text-center">
            <p className="text-gray-600 mb-6">
                您的账号已被管理员禁用，暂时无法访问系统。如有疑问，请联系客服或管理员进行处理。
            </p>
            <button
                onClick={onClose}
                className="w-full bg-danger text-white py-2.5 rounded-lg hover:bg-danger/90 transition-colors font-medium"
            >
                我知道了
            </button>
        </div>
    </ModalWrapper>
);

export default DisabledAccountModal;
