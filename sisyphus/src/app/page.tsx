"use client"
import FabMenu from "@/components/organisms/fixedFloatingActionButton";
import HeroSection from "@/components/organisms/hero";
import { useUser } from "@/components/molecules/userContext";
import Sidebar from "@/components/sidebar";

export default function Home() {
  const { isPatientMode, setIsPatientMode } = useUser();

  const switchUser = () => {
    setIsPatientMode((prev: boolean) => !prev)
  }

  return (
    <div className="w-full  bg-white min-h-screen  relative ">
      <Sidebar/>
      <button onClick={switchUser} className=" px-4 py-2 bg-blue-700 text-white rounded-sm cursor-pointer " > {isPatientMode ? "Patient mode" : "Trainer mode"} </button>
      <HeroSection />
      <FabMenu />
    </div>
  );
}
