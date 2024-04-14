import { chainName, MarketAddress } from "../config";
import { useChain } from "@cosmos-kit/react";
import {
  Box,
  Button,
  Skeleton,
  VStack,
  useColorModeValue,
  Flex,
  Text,
  Popover,
  PopoverTrigger,
  PopoverContent,
  PopoverHeader,
  PopoverArrow,
  PopoverCloseButton,
  PopoverFooter,
  PopoverBody,
  Tooltip,
} from "@chakra-ui/react";
import { ArrayOfTupleOfUint64AndListItemData, ListItemData, ExecuteMsg } from "../config/market";
import { useState } from "react";
import React from "react";
import { Coin } from "@cosmjs/amino";
import { useMarketTx } from "../hook";

const BuyButton = ({
    id,
    data
  }: {
    id:number;
    data:ListItemData
})=>{
    const initialFocusRef = React.useRef()
    const { assets, address } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const {tx} = useMarketTx(chainName,"unibi",MarketAddress)
    const getdenomMap =() => {
        let map = new Map<String,String>()
        assets.assets.map((value)=>{
            map.set(value.denom_units[0].denom,value.name)
        })
        return map;
    }

    const handleBuy = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {
            buy:{
            id:id
            }
        }
        const funds:Coin[]=[{denom:"unibi",amount:data.price}]
        console.log(msg)
        await tx(msg,{},funds);
        setIsSubmitting(false)
      }
  

    return (
        <Popover
          initialFocusRef={initialFocusRef}
          placement='bottom'
        >{({ isOpen, onClose }) => (
        <>
          <PopoverTrigger>
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>Number(data.expires)/1000000) } size='lg'>Buy it</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
            You will buy the Option with id: {id} 
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            <VStack align="start" fontWeight="bold" fontSize={{ md: 'lg' }}  mb={1}>
                <Flex justify="space-between" w = "full"><Text> You need to pay:</Text>
                 <Text>
                  {address ? (
                    `${Number(data.price)/1000000} NIBI`
                  ) : (
                    'Connect wallet'
                  )}
                 </Text>
                </Flex> 
           </VStack>        

            </PopoverBody>

            <PopoverFooter
              border='0'
              display='flex'
              alignItems='center'
              justifyContent='center'
              pb={4}
            >
            <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleBuy();}}>
                  Confirm
            </Button>
            </PopoverFooter>
          </PopoverContent>
            </>
          )}
        </Popover>       
    )

}

const OptionCard = ({ data, id }: { data: ListItemData; id: number }) => {
  const { assets, address } = useChain(chainName);

  const getDenomMap = () => {
    const map = new Map<string, string>();
    assets.assets.forEach((value) => {
      map.set(value.denom_units[0].denom, value.name);
    });
    return map;
  };

  return (
    <Box className="option-card">
      <Flex justify="space-between" align="center" mb={4}>
        <Text fontSize="xl" fontWeight="bold">
          Option ID: {id}
        </Text>
        <BuyButton id={id} data={data} />
      </Flex>

      <VStack align="start" spacing={4}>
        <Flex justify="space-between" w="full">
          <Text fontWeight="bold">Creator:</Text>
          <Text>{data.creator}</Text>
        </Flex>

        <Flex justify="space-between" w="full">
          <Text fontWeight="bold">Owner:</Text>
          <Text>{data.owner}</Text>
        </Flex>

        <Flex justify="space-between" w="full">
          <Text fontWeight="bold">Collateral:</Text>
          <Text>
            {address ? (
              `${Number(data.collateral.amount) / 1000000} ${getDenomMap().get(data.collateral.denom)}`
            ) : (
              <Tooltip label="Connect wallet to see the value" placement="top">
                <Text cursor="default">-</Text>
              </Tooltip>
            )}
          </Text>
        </Flex>

        <Flex justify="space-between" w="full">
          <Text fontWeight="bold">Counter Offer:</Text>
          <Text>
            {address ? (
              `${Number(data.counter_offer.amount) / 1000000} ${getDenomMap().get(data.counter_offer.denom)}`
            ) : (
              <Tooltip label="Connect wallet to see the value" placement="top">
                <Text cursor="default">-</Text>
              </Tooltip>
            )}
          </Text>
        </Flex>

        <Flex justify="space-between" w="full">
          <Text fontWeight="bold">Expiration Date:</Text>
          <Text>{new Date(Number(data.expires) / 1000000).toDateString()}</Text>
        </Flex>

        <Flex justify="space-between" w="full" fontSize="2xl" color="blue.500">
          <Text fontWeight="bold">Price:</Text>
          <Text>
            {address ? (
              `${Number(data.price) / 1000000} NIBI`
            ) : (
              <Tooltip label="Connect wallet to see the value" placement="top">
                <Text cursor="default">-</Text>
              </Tooltip>
            )}
          </Text>
        </Flex>
      </VStack>
    </Box>
  );
};

export const BuyOptionsList = () => {
  const { address, getCosmWasmClient } = useChain(chainName);
  const [datas, setData] = useState<ArrayOfTupleOfUint64AndListItemData>();

  const handleQueryOwnerList = async () => {
    if (!address) {
      alert("Please connect your wallet!");
      return;
    }

    let client: Awaited<ReturnType<typeof getCosmWasmClient>>;
    try {
      client = await getCosmWasmClient();
    } catch (e: any) {
      console.error(e);
      return;
    }

    const options: Promise<ArrayOfTupleOfUint64AndListItemData> = client.queryContractSmart(MarketAddress, {
      list_items: {},
    });

    options.then((value) => {
      setData(value);
      console.log(value);
    });
  };

  return (
    <Box className="buy-options-list">
      <Flex justify="space-between" align="center" mb={6}>
        <Text fontSize="2xl" fontWeight="bold">
          Buy Options
        </Text>
        <Button onClick={handleQueryOwnerList} colorScheme="blue" size="lg">
          Refresh
        </Button>
      </Flex>

      {datas ? (
        <VStack spacing={6}>
          {datas.map(([id, data]) => (
            <OptionCard key={id} data={data} id={id} />
          ))}
        </VStack>
      ) : (
        <Skeleton height={200} />
      )}
    </Box>
  );
};

