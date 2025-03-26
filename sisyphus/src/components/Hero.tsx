"use client";

import Image from "next/image";
import { Button } from "@/components/ui/button";
import { Apple, Store } from "lucide-react";
import { MessageSquare } from "lucide-react";
import passport from "../../public/passport.webp";

const HeroSection = () => {
  return (
    <section className="relative w-full h-[400px] flex  items-center justify-around bg-gradient-to-r from-teal-800 to-teal-100 rounded-xl p-6 overflow-hidden">
      {/* Left Section */}
      <div className="flex text-white max-w-lg">
        <div className=" rounded-full flex items-center justify-center">
          <MessageSquare size={80} className="" />

          {/* <Image src="/icon.svg" alt="Logo" width={24} height={24} /> */}
        </div>
        <div className="flex items-start flex-col">
          <h2 className="text-lg font-semibold">Home</h2>
          <span className="text-sm mt-1 opacity-80 p-1 px-2 border rounded-full">
            ℹ Generic logo
          </span>
        </div>
      </div>

      {/* Right Section */}
      <div className="flex-1 rounded-xl p-6 relative max-w-lg">
        <h1 className="text-2xl font-bold text-black">Hairimse</h1>
        <h3 className="text-sm font-semibold text-gray-600">
          Fugh Matte For Rfilestde
        </h3>
        <p className="text-gray-500 mt-2">
          Meta Crossplatform solts fre aponate vth the licor heavell to act your
          readipts.
        </p>

        <div className="flex space-x-4 mt-4 justify-between">
          <div className="flex items-center space-x-2 text-teal-500">
            <MessageSquare />
            <span>Lorem, ipsum dolor.</span>
          </div>
          <div className="flex space-x-2">
            <Button className="flex items-center space-x-2 bg-black text-white px-4 py-2 rounded-lg">
              <Apple size={16} /> <span>App Store</span>
            </Button>
            <Button className="flex items-center space-x-2 bg-black text-white px-4 py-2 rounded-lg">
              <Store size={16} /> <span>Fristope</span>
            </Button>
          </div>
        </div>

        <div className="absolute -top-25 right-6 w-45 h-45 bg-green-500 rounded-full overflow-hidden border-4">
          <Image src={passport} alt="User" layout="fill" objectFit="cover" />
        </div>
      </div>
    </section>
  );
};

export default HeroSection;
