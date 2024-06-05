import { create } from "zustand";

export const useSession = create((set, get) => ({
  identity: null,
  principal: null,
  isConnected: false,
  isConnecting: true,
  setIdentity: (identity, principal) =>
    set({ identity, principal, isConnected: true, isConnecting: false }),
  setConnecting: (isConnecting) => set({ isConnecting }),
  logout: () => set({ identity: null, principal: null, isConnected: false }),
}));