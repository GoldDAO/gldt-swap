import { MouseEventHandler } from "react";
import { useAtomValue } from "jotai";
import { useAuth as useAuthIK } from "@nfid/identitykit/react";
import { Actor, ActorSubclass } from "@dfinity/agent";

import { stateAtom } from "../atoms";

export const useAuth = () => {
  const { connect: connectIK, disconnect: disconnectIK } = useAuthIK();

  const state = useAtomValue(stateAtom);

  const connect: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault();
    e.stopPropagation();
    connectIK();
  };

  const disconnect = () => {
    disconnectIK();
  };

  const createActor = (
    canister: string,
    options: { authenticated: boolean } = { authenticated: false }
  ): ActorSubclass => {
    const { canisterId, idlFactory } = state.canisters[canister];

    const actor = Actor.createActor(idlFactory, {
      agent: options.authenticated
        ? state.authenticatedAgent
        : state.unauthenticatedAgent,
      canisterId,
    });

    // console.groupCollapsed(
    //   "Create actor fn w/ details on agent and actor created"
    // );
    // console.log({ canister });
    // console.log({ authenticated: options.authenticated });
    // console.log({ authenticatedAgent: state.authenticatedAgent });
    // console.log({ unauthenticatedAgent: state.unauthenticatedAgent });
    // console.log({ actor });
    // console.groupEnd();

    return actor;
  };

  return {
    isConnected: state.isConnected,
    isConnecting: state.isConnecting,
    principalId: state.principalId,
    connect,
    disconnect,
    createActor,
  };
};
