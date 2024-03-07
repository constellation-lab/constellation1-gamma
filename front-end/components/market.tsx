import { chainName,contractAddress } from "../config"
import { useChain } from '@cosmos-kit/react';
import { Box, Button,Tab,Tabs,TabList,TabPanel,TabPanels} from "@chakra-ui/react";
import { ArrayOfTupleOfUint64AndData } from "../config/constellation";
import { useState } from "react";
import { Data } from '../config/constellation/Constellation.types';
import React from "react";
import { ExecuteMsg } from "../config/constellation/Constellation.types";
import { Coin } from "@cosmjs/amino";
import { useTx } from "../hook";
import { AllOptionList } from "./bid_list";



export const MarketOptionsList = ()=>{
    const { address, getCosmWasmClient } = useChain(chainName);
    const [datas, setData] = useState<ArrayOfTupleOfUint64AndData>() 
    const [isSubmitting, setIsSubmitting] = useState(false);
    const {tx} = useTx(chainName,"unibi",contractAddress)

    const handleQueryOwnerList =async () =>{
        if (!address){
            alert("please connect your wallet!!")
            return
        }
        let client: Awaited<ReturnType<typeof getCosmWasmClient>>;
        try {
          client = await getCosmWasmClient();
        } catch (e: any) {
          console.error(e);
          return;
        }
        let options:Promise<ArrayOfTupleOfUint64AndData>  = client.queryContractSmart(contractAddress,{
            "market_options":{}
          })
        options.then((value)=>{setData(value);console.log(value)})
    } 
    const handleApprove = async () => {
      setIsSubmitting(true)
      let msg:ExecuteMsg = {
          approve:{
          spender:"nibi1hntfu45etpkdf8prq6p6la9tsnk3u3muf5378kds73c7xd4qdzyscka8pj"
          }
      }
      const funds:Coin[]=[]
      console.log(msg)
      await tx(msg,{},funds);
      setIsSubmitting(false)
    }



    return(
    <Box>
        <Button onClick={handleApprove} w="full" justifyContent="center" >set Approve for market</Button>
        <Tabs>
      <TabList>
        <Tab>All Options</Tab>
        <Tab>Buy Option</Tab>
        <Tab>List your option</Tab>
        <Tab>Unlist your option</Tab>
      </TabList>
      <TabPanels>
        <TabPanel><AllOptionList/></TabPanel>
        <TabPanel></TabPanel>
        <TabPanel></TabPanel>
        <TabPanel></TabPanel>
      </TabPanels>
    </Tabs>
    </Box>
    )
}