import { chainName, contractAddress, MarketAddress } from "../config";
import { useChain } from "@cosmos-kit/react";
import { Box, Button, Tab, Tabs, TabList, TabPanel, TabPanels } from "@chakra-ui/react";
import { ArrayOfTupleOfUint64AndData } from "../config/constellation";
import { useState } from "react";
import { Data } from "../config/constellation/Constellation.types";
import React from "react";
import { ExecuteMsg } from "../config/constellation/Constellation.types";
import { Coin } from "@cosmjs/amino";
import { useTx } from "../hook";
import { AllOptionList } from "./bid_list";
import { BuyOptionsList } from "./Buy_option_page";
import { ListOptionList } from "./List_option_page";
import { ManageOptionList } from "./unlist_page";

export const MarketOptionsList = () => {
  const { address, getCosmWasmClient } = useChain(chainName);
  const [datas, setData] = useState<ArrayOfTupleOfUint64AndData>();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const { tx } = useTx(chainName, "unibi", contractAddress);

  const handleApprove = async () => {
    setIsSubmitting(true);
    const msg: ExecuteMsg = {
      approve: {
        spender: MarketAddress,
      },
    };
    const funds: Coin[] = [];
    console.log(msg);
    await tx(msg, {}, funds);
    setIsSubmitting(false);
  };

  return (
    <Box className="market-options-list">
      <Box className="market-options-header" mb={6}>
        <Button
          className="approve-button"
          onClick={handleApprove}
          isLoading={isSubmitting}
          colorScheme="blue"
          size="lg"
          mb={4}
        >
          Set Approve for Market
        </Button>
      </Box>

      <Tabs variant="soft-rounded" colorScheme="blue">
        <TabList className="tab-list" mb={6}>
          <Tab _selected={{ bg: "blue.500", color: "white" }}>Buy an Option</Tab>
          <Tab _selected={{ bg: "blue.500", color: "white" }}>List Your Option</Tab>
          <Tab _selected={{ bg: "blue.500", color: "white" }}>Unlist Your Option</Tab>
        </TabList>

        <TabPanels>
          <TabPanel>
            <BuyOptionsList />
          </TabPanel>
          <TabPanel>
            <ListOptionList />
          </TabPanel>
          <TabPanel>
            <ManageOptionList />
          </TabPanel>
        </TabPanels>
      </Tabs>
    </Box>
  );
};