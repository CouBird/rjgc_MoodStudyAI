import React from "react";

const ModalWrapper = ({ isOpen, onClose, maxWidth = "max-w-md", children }) => {
  if (!isOpen) return null;
  const handleOverlayClick = (e) => {
    if (e.target === e.currentTarget) onClose();
  };
  return (
    <div
      onClick={handleOverlayClick}
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-50 p-4 backdrop-blur-xs transition-opacity"
    >
      <div
        className={`bg-white rounded-2xl shadow-2xl w-full ${maxWidth} max-h-[85vh] flex flex-col overflow-hidden transform scale-100 transition-all animate-slide-up`}
      >
        {children}
      </div>
    </div>
  );
};
export default ModalWrapper;
