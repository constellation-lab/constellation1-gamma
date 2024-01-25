/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { UseQueryOptions, useQuery } from "react-query";
import { InstantiateMsg, ExecuteMsg, Uint128, Coin, QueryMsg, Addr, State, Timestamp, Uint64, ArrayOfTupleOfUint64AndData, Data } from "./Consteallation.types";
import { ConsteallationQueryClient } from "./Consteallation.client";
export interface ConsteallationReactQuery<TResponse, TData = TResponse> {
  client: ConsteallationQueryClient | undefined;
  options?: UseQueryOptions<TResponse, Error, TData>;
}
export interface ConsteallationOwnerOptionsQuery<TData> extends ConsteallationReactQuery<ArrayOfTupleOfUint64AndData, TData> {
  args: {
    addr: string;
  };
}
export function useConsteallationOwnerOptionsQuery<TData = ArrayOfTupleOfUint64AndData>({
  client,
  args,
  options
}: ConsteallationOwnerOptionsQuery<TData>) {
  return useQuery<ArrayOfTupleOfUint64AndData, Error, TData>(["consteallationOwnerOptions", client?.contractAddress, JSON.stringify(args)], () => client ? client.ownerOptions({
    addr: args.addr
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ConsteallationCreateorOptionsQuery<TData> extends ConsteallationReactQuery<ArrayOfTupleOfUint64AndData, TData> {
  args: {
    addr: string;
  };
}
export function useConsteallationCreateorOptionsQuery<TData = ArrayOfTupleOfUint64AndData>({
  client,
  args,
  options
}: ConsteallationCreateorOptionsQuery<TData>) {
  return useQuery<ArrayOfTupleOfUint64AndData, Error, TData>(["consteallationCreateorOptions", client?.contractAddress, JSON.stringify(args)], () => client ? client.createorOptions({
    addr: args.addr
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ConsteallationMaketOptionsPageeQuery<TData> extends ConsteallationReactQuery<ArrayOfTupleOfUint64AndData, TData> {
  args: {
    amount: number;
    key: number;
  };
}
export function useConsteallationMaketOptionsPageeQuery<TData = ArrayOfTupleOfUint64AndData>({
  client,
  args,
  options
}: ConsteallationMaketOptionsPageeQuery<TData>) {
  return useQuery<ArrayOfTupleOfUint64AndData, Error, TData>(["consteallationMaketOptionsPagee", client?.contractAddress, JSON.stringify(args)], () => client ? client.maketOptionsPagee({
    amount: args.amount,
    key: args.key
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ConsteallationMarketOptionsQuery<TData> extends ConsteallationReactQuery<ArrayOfTupleOfUint64AndData, TData> {}
export function useConsteallationMarketOptionsQuery<TData = ArrayOfTupleOfUint64AndData>({
  client,
  options
}: ConsteallationMarketOptionsQuery<TData>) {
  return useQuery<ArrayOfTupleOfUint64AndData, Error, TData>(["consteallationMarketOptions", client?.contractAddress], () => client ? client.marketOptions() : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ConsteallationGetOptionByidQuery<TData> extends ConsteallationReactQuery<Data, TData> {
  args: {
    id: number;
  };
}
export function useConsteallationGetOptionByidQuery<TData = Data>({
  client,
  args,
  options
}: ConsteallationGetOptionByidQuery<TData>) {
  return useQuery<Data, Error, TData>(["consteallationGetOptionByid", client?.contractAddress, JSON.stringify(args)], () => client ? client.getOptionByid({
    id: args.id
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ConsteallationOptionsPageQuery<TData> extends ConsteallationReactQuery<ArrayOfTupleOfUint64AndData, TData> {
  args: {
    amount: number;
    key: number;
  };
}
export function useConsteallationOptionsPageQuery<TData = ArrayOfTupleOfUint64AndData>({
  client,
  args,
  options
}: ConsteallationOptionsPageQuery<TData>) {
  return useQuery<ArrayOfTupleOfUint64AndData, Error, TData>(["consteallationOptionsPage", client?.contractAddress, JSON.stringify(args)], () => client ? client.optionsPage({
    amount: args.amount,
    key: args.key
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ConsteallationOptionsQuery<TData> extends ConsteallationReactQuery<ArrayOfTupleOfUint64AndData, TData> {}
export function useConsteallationOptionsQuery<TData = ArrayOfTupleOfUint64AndData>({
  client,
  options
}: ConsteallationOptionsQuery<TData>) {
  return useQuery<ArrayOfTupleOfUint64AndData, Error, TData>(["consteallationOptions", client?.contractAddress], () => client ? client.options() : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface ConsteallationConfigQuery<TData> extends ConsteallationReactQuery<State, TData> {}
export function useConsteallationConfigQuery<TData = State>({
  client,
  options
}: ConsteallationConfigQuery<TData>) {
  return useQuery<State, Error, TData>(["consteallationConfig", client?.contractAddress], () => client ? client.config() : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}