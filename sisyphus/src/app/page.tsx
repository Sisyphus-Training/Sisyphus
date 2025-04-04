
import Image from "next/image";

export default function Home() {
  return (
    <div className="w-full  bg-[#0a0b14] h-[30vh]  relative ">

      <div className="bg-gradient-to-r from-[#00b8ff] to-[#41d98d] h-[35%] w-full "  >

    </div>

    <div className=" absolute top-[50%] translate-y-[-50%] left-[5%] flex items-center space-x-4">
      <div className="relative w-16 h-16 rounded-full overflow-hidden bg-white flex-shrink-0">
        <Image
          src="/trainer.jpg"
          alt="Profile picture"
          width={64}
          height={64}
          className="object-cover h-full w-full "
        />
      </div>
      <div>
        <h2 className="text-white text-xl font-bold">John Doe</h2>
        <p className="text-gray-300 text-sm">Certified Personal Trainer</p>
      </div>
    </div>

  </div>
  );
}
