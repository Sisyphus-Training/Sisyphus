"use client"

import React, { createContext, useContext, useState } from "react";

interface UserContextType {
  isPatientMode: boolean;
  setIsPatientMode: (mode: boolean) => void;
}

const UserContext = createContext<UserContextType | undefined>(undefined);

export const UserProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [isPatientMode, setIsPatientMode] = useState<boolean>(false);

  return (
    <UserContext.Provider value={{ isPatientMode, setIsPatientMode }}>
      {children}
    </UserContext.Provider>
  );
};

export const useUser = (): UserContextType => {
  const context = useContext(UserContext);
  if (!context) {
    throw new Error("useUser must be used within a UserProvider");
  }
  return context;
};
