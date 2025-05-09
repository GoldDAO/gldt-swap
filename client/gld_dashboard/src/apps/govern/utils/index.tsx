import { Ledger } from "@services/ledger/utils/interfaces";
export interface Neuron {
  id: string;
  reward: bigint;
  reward_usd: number;
}

export interface Reward {
  id: Ledger;
  name: string;
  label: string;
  canister_id: string;
  is_selected: boolean;
  is_claimable: boolean;
  amount: bigint;
  amount_usd: number;
  neurons: Neuron[];
}

import {
  GOLDAO_LEDGER_CANISTER_ID,
  ICP_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
  WTN_LEDGER_CANISTER_ID,
} from "@constants";

export interface Token {
  id: Ledger;
  name: string;
  label: string;
  canisterId: string;
}

export const TokensList: Token[] = [
  {
    id: "goldao",
    name: "GOLDAO",
    label: "GOLDAO",
    canisterId: GOLDAO_LEDGER_CANISTER_ID,
  },
  {
    id: "icp",
    name: "ICP",
    label: "Internet Computer",
    canisterId: ICP_LEDGER_CANISTER_ID,
  },
  {
    id: "ogy",
    name: "OGY",
    label: "Origyn",
    canisterId: OGY_LEDGER_CANISTER_ID,
  },
  {
    id: "wtn",
    name: "WTN",
    label: "Waterneuron",
    canisterId: WTN_LEDGER_CANISTER_ID,
  },
];
