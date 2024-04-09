import { chainName,contractAddress, MarketAddress } from "../config"
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
import { BuyOptionsList } from "./Buy_option_page";
import { ListOptionList } from "./List_option_page";
import { ManageOptionList } from "./unlist_page";



export const MarketOptionsList = ()=>{
    const { address, getCosmWasmClient } = useChain(chainName);
    const [datas, setData] = useState<ArrayOfTupleOfUint64AndData>() 
    const [isSubmitting, setIsSubmitting] = useState(false);
    const {tx} = useTx(chainName,"unibi",contractAddress)

    const handleApprove = async () => {
      setIsSubmitting(true)
      let msg:ExecuteMsg = {
          approve:{
            spender:MarketAddress
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
        {/*<Tab>All Options</Tab>*/}
        <Tab>Buy an option</Tab>
        <Tab>List your option</Tab>
        <Tab>Unlist your option</Tab>
      </TabList>
      <TabPanels>
        {/* <TabPanel><AllOptionList/></TabPanel> */}
        <TabPanel><BuyOptionsList/></TabPanel>
        <TabPanel><ListOptionList/></TabPanel>
        <TabPanel><ManageOptionList/></TabPanel>
      </TabPanels>
    </Tabs>
    </Box>
    )
}import { chainName,contractAddress, MarketAddress } from "../config"
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
import { BuyOptionsList } from "./Buy_option_page";
import { ListOptionList } from "./List_option_page";
import { ManageOptionList } from "./unlist_page";



export const MarketOptionsList = ()=>{
    const { address, getCosmWasmClient } = useChain(chainName);
    const [datas, setData] = useState<ArrayOfTupleOfUint64AndData>() 
    const [isSubmitting, setIsSubmitting] = useState(false);
    const {tx} = useTx(chainName,"unibi",contractAddress)

    const handleApprove = async () => {
      setIsSubmitting(true)
      let msg:ExecuteMsg = {
          approve:{
            spender:MarketAddress
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
        {/*<Tab>All Options</Tab>*/}
        <Tab>Buy Option</Tab>
        <Tab>List your option</Tab>
        <Tab>Unlist your option</Tab>
      </TabList>
      <TabPanels>
        {/* <TabPanel><AllOptionList/></TabPanel> */}
        <TabPanel><BuyOptionsList/></TabPanel>
        <TabPanel><ListOptionList/></TabPanel>
        <TabPanel><ManageOptionList/></TabPanel>
      </TabPanels>
    </Tabs>
    </Box>
    )
}