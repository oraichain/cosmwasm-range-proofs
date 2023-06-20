import {Addr, Binary, Boolean} from "./types";
export interface InstantiateMsg {
  n: number;
}
export type ExecuteMsg = {
  set_owner: {
    new_owner: Addr;
  };
};
export type QueryMsg = {
  verify_proof: {
    challenge: Binary;
    commitment: Binary;
    proof_raw: Binary;
  };
};