"use client"
import FabMenu from "@/components/fixedFloatingActionButton";
import HeroSection from "@/components/Hero";
import { useUser } from "@/context/UserContext";



export default function Home() {
  const { isPatientMode, setIsPatientMode } = useUser();


  const switchUser = (e) => {
    setIsPatientMode((prev) => !prev)
  }


  return (
    <div className="w-full  bg-white min-h-screen  relative ">


      <button onClick={switchUser} className=" px-4 py-2 bg-blue-700 text-white rounded-sm cursor-pointer " > {isPatientMode ? "Patient mode" : "Trainer mode"} </button>



      <HeroSection />
      <FabMenu />




    </div>

  );
}
