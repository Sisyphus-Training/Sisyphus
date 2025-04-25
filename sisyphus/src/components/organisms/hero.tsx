"use client";

import Image from "next/image";
import { Button } from "@/components/atoms/button";
import { Apple, Store } from "lucide-react";
import { MessageSquare } from "lucide-react";
import logo from "../../../public/logo.jpg";
import { useUser } from "@/components/molecules/userContext";
import { patientData, trainerData, Users } from "@/components/atoms/userData";
import Sidebar from "../sidebar";

const HeroSection = () => {
  const { isPatientMode } = useUser();
  const currentUser: Users = isPatientMode ? patientData : trainerData;

  return (
    <section className="relative w-full min-h-[400px] flex flex-col md:flex-row items-center justify-around bg-gradient-to-r from-[#141f25] to-cyan-400 rounded-xl p-4 md:p-6 overflow-hidden">
      {/* Left Section */}
      <div className="flex text-white w-full md:max-w-lg space-x-2 justify-center md:justify-start items-center mb-6 md:mb-0">
        <div className="rounded-full flex items-center justify-center">
          <Image
            src={logo}
            alt="Logo"
            width={70}
            height={70}
            className="rounded-full w-16 h-16 md:w-20 md:h-20"
          />
        </div>
        <div className="flex items-start flex-col">
          <h2 className="text-base md:text-lg font-semibold">Home</h2>
          <span className="text-xs md:text-sm mt-1 opacity-80 p-1 px-2 border rounded-full">
            ℹ Generic logo
          </span>
        </div>
      </div>

      {/* Right Section */}
      <div className="w-full md:flex-1 rounded-xl p-4 md:p-6 relative max-w-lg backdrop-blur-sm">
        <h1 className="text-xl md:text-2xl font-bold text-black">{currentUser.name} </h1>
        <h3 className="text-xs md:text-sm font-semibold text-gray-600">
          {currentUser.title}
        </h3>
        <p className="text-gray-900 mt-2 text-sm md:text-base ">
          Meta Crossplatform solts fre aponate vth the licor heavell to act your
          readipts.
        </p>

        <div className="flex flex-col md:flex-row md:space-x-4 mt-4 justify-between gap-3">
          <div className="flex items-center space-x-2 text-teal-500 text-sm md:text-base">
            <MessageSquare size={16} />
            <span>Lorem, ipsum dolor.</span>
          </div>
          <div className="flex space-x-2 justify-center md:justify-end">
            <Button className="flex items-center space-x-2 bg-[##49B848] text-white px-3 py-1 md:px-4 md:py-2 rounded-lg text-xs md:text-sm">
              <Apple size={14} /> <span>App Store</span>
            </Button>
            <Button className="flex items-center space-x-2 bg-[##49B848] text-white px-3 py-1 md:px-4 md:py-2 rounded-lg text-xs md:text-sm">
              <Store size={14} /> <span>Fristope</span>
            </Button>
          </div>
        </div>

        <div className="absolute -top-12 -right-8 md:-top-15 md:right-16 w-24 h-24 md:w-35 md:h-35 bg-green-500 rounded-full overflow-hidden border-4">
          <Image
            src={currentUser.profilePic}
            alt="User"
            layout="fill"
            objectFit="cover"
            className="object-cover"
          />
        </div>
      </div>
    </section>
  );
};

export default HeroSection;
