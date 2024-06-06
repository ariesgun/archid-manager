/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.10.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export type Addr = string;
export interface InstantiateMsg {
  archid_registry_addr: Addr;
  count: number;
  cw721_archid_addr: Addr;
  denom: string;
}
export type ExecuteMsg = {
  increment: {};
} | {
  reset: {
    count: number;
  };
} | {
  mint_domain: {
    domain_name: string;
  };
} | {
  renew_domain: {
    domain_name: string;
  };
} | {
  schedule_auto_renew: {
    domain_name: string;
  };
} | {
  set_default: {
    domain_name: string;
  };
};
export type QueryMsg = {
  get_count: {};
} | {
  query_errors: {};
} | {
  query_domain_default: {
    address: Addr;
  };
};
export interface GetCountResponse {
  count: number;
}
export interface DomainDefaultResponse {
  domain_id: string;
}
export interface QueryErrorsResponse {
  errors: SudoError[];
}
export interface SudoError {
  contract_address: string;
  error_code: number;
  error_message: string;
  input_payload: string;
  module_name: string;
}