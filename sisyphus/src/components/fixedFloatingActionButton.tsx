"use client";

import { useState } from "react";
import { Plus, User, Dumbbell, Clock, Utensils } from "lucide-react";

const FabMenu = () => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="fixed bottom-6 right-6 flex flex-col items-end space-y-2">
      {/* Action Buttons */}
      <div
        className={`flex flex-col items-end space-y-2 transition-all ${
          isOpen
            ? "opacity-100 translate-y-0"
            : "opacity-0 translate-y-4 pointer-events-none"
        }`}
      >
        <button className="w-12 h-12 rounded-full bg-blue-500 flex items-center justify-center shadow-lg">
          <User className="text-white" size={20} />
        </button>
        <button className="w-12 h-12 rounded-full bg-green-500 flex items-center justify-center shadow-lg">
          <Dumbbell className="text-white" size={20} />
        </button>
        <button className="w-12 h-12 rounded-full bg-purple-500 flex items-center justify-center shadow-lg">
          <Clock className="text-white" size={20} />
        </button>
        <button className="w-12 h-12 rounded-full bg-orange-500 flex items-center justify-center shadow-lg">
          <Utensils className="text-white" size={20} />
        </button>
      </div>

      {/* Floating Action Button */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-14 h-14 rounded-full bg-blue-500 flex items-center justify-center shadow-xl transition-transform transform"
      >
        <Plus
          className={`text-white transition-transform ${
            isOpen ? "rotate-45" : "rotate-0"
          }`}
          size={24}
        />
      </button>
    </div>
  );
};

export default FabMenu;
