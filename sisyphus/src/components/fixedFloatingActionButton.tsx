"use client";

import { useState } from "react";
import { Plus, User, Dumbbell, Clock, Utensils } from "lucide-react";

const FabMenu = () => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="fixed bottom-6 right-6 flex flex-col items-center space-y-2">
      {/* Action Buttons */}
      <div
        className={`flex flex-col items-end space-y-2 transition-all ${
          isOpen
            ? "opacity-100 translate-y-0"
            : "opacity-0 translate-y-4 pointer-events-none"
        }`}
      >
        <div className="relative flex items-center group">
          <span className="absolute right-14 bg-gray-800 text-white px-2 py-1 rounded-md text-sm opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Profile
          </span>
          <button className="w-12 h-12 rounded-full bg-[#3b82f6] flex items-center justify-center shadow-lg hover:scale-110 transition-transform">
            <User className="text-white" size={20} />
          </button>
        </div>
        <div className="relative flex items-center group">
          <span className="absolute right-14 bg-gray-800 text-white px-2 py-1 rounded-md text-sm opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Workout
          </span>
          <button className="w-12 h-12 rounded-full bg-[#22c55e] flex items-center justify-center shadow-lg hover:scale-110 transition-transform">
            <Dumbbell className="text-white" size={20} />
          </button>
        </div>
        <div className="relative flex items-center group">
          <span className="absolute right-14 bg-gray-800 text-white px-2 py-1 rounded-md text-sm opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Schedule
          </span>
          <button className="w-12 h-12 rounded-full bg-[#a855f7] flex items-center justify-center shadow-lg hover:scale-110 transition-transform">
            <Clock className="text-white" size={20} />
          </button>
        </div>
        <div className="relative flex items-center group">
          <span className="absolute right-14 bg-gray-800 text-white px-2 py-1 rounded-md text-sm opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Meals
          </span>
          <button className="w-12 h-12 rounded-full bg-[#ea580c] flex items-center justify-center shadow-lg hover:scale-110 transition-transform">
            <Utensils className="text-white" size={20} />
          </button>
        </div>
      </div>

      {/* Floating Action Button */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-14 h-14 rounded-full bg-[#00b0f0] flex items-center justify-center shadow-xl transition-transform transform hover:scale-110"
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
