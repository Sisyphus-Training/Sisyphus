import Image from "next/image";
import FabMenu from "@/components/fixedFloatingActionButton";
import HeroSection from "@/components/Hero";

export default function Home() {
  return (
    <div>
      <h1>Home</h1>
      <HeroSection />
      <FabMenu />
    </div>
  );
}
