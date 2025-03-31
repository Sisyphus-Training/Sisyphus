import TrainerProfile from "@/components/trainer/TrainerProfile";
import ActivePatientInfo from "@/components/trainer/ActivePatientInfo";

type Role = "trainer" | "patient";

interface TrainerDashboardProps {
  role?: Role;
}

const TrainerDashboard = ({ role = "patient" }: TrainerDashboardProps) => {
  return (
    <main className="min-h-screen p-8 bg-white dark:bg-slate-900 text-slate-900 dark:text-white">
      <h1 className="text-2xl font-bold mb-6">Trainer Dashboard</h1>
      <TrainerProfile />

      {role === "trainer" ? (
        <ActivePatientInfo />
      ) : (
        <div className="space-y-4">
          <h2 className="text-xl font-semibold text-white">Your Routine</h2>
          <div className="bg-slate-800 p-4 rounded-lg text-white">
            <p className="font-medium">🧘 Stretching - 10 mins</p>
            <p className="font-medium">🏋️ Strength Training - 20 mins</p>
            <p className="font-medium">🚴 Cardio - 15 mins</p>
          </div>
          <div className="text-sm text-slate-400">
            Last updated: 2 hours ago
          </div>
        </div>
      )}
    </main>
  );
};

export default TrainerDashboard;
