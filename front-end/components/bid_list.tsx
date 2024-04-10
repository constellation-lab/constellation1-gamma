import { chainName,contractAddress,MarketAddress } from "../config"
import { useChain } from '@cosmos-kit/react';
import { Box, Button,Skeleton,VStack,useColorModeValue,Flex,Text,Popover,PopoverTrigger,PopoverContent,PopoverHeader,PopoverArrow,PopoverCloseButton,Editable,EditablePreview,EditableInput,Select,
    PopoverFooter,PopoverBody, Spacer} from "@chakra-ui/react";
import { ArrayOfTupleOfUint64AndData } from "../config/constellation";
import { useState } from "react";
import { Data } from '../config/constellation/Constellation.types';
import React from "react";
import { ExecuteMsg,Addr,Bid } from "../config/market/ContellaMarket.types";
import { useMarketTx } from "../hook";
import { Coin } from '@cosmjs/amino';


const BidButton = ({
  id,
}: {
  id:number;
})=>{
  const initialFocusRef = React.useRef()
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [price, setPrice] = useState(0)
  const {tx} = useMarketTx(chainName,"unibi",MarketAddress)

  const handleExecuteOption = async () => {
      setIsSubmitting(true)
      let funds:Coin[]=[{amount:(price*1000000).toString(), denom: 'unibi'}];

      let msg:ExecuteMsg = {
        set_bid: {
          expires: Math.floor((Date.now()+800*24*60*60*1000)/1000),
          id: id
        }
      }
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
          <Button isLoading={isSubmitting} colorScheme="primary">Bid</Button>
        </PopoverTrigger>
        <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
          <PopoverHeader pt={4} fontWeight='bold' border='10' >
            List Option ID: {id} 
          </PopoverHeader>
          <PopoverArrow bg='blue.800' />
          <PopoverCloseButton />
          <PopoverBody>
          Set price:
          <Box flex={1}>
              <Editable
              variant="unstyled"
              fontSize={{ base: 'lg', sm: 'xl' }}
              fontWeight="bold"
              mb={{ base: 1, sm: 2 }}
              placeholder="0"
              >
                  <EditablePreview />
                  <EditableInput
                  type="number"
                  min="0"
                  defaultValue="0"
                  onChange={(e) => {
                  const value = e.target.value;
                  const floatRegex =
                      /(0{0,1}[.]d*)(d+([.]d*)?(e[+-]?d+)?|[.]d+(e[+-]?d+)?)/g;
                  const floatCheck = value.match(floatRegex);
                  if (floatCheck !== null) {
                      setPrice(Number(value))
                      return value;
                   }
                  setPrice(parseFloat(value))
                  return (e.target.value = parseFloat(value).toString());
                  }}
                  _focus={{ boxShadow: 'none' }}
              />
              </Editable>
          </Box>
          </PopoverBody>

          <PopoverFooter
            border='0'
            display='flex'
            alignItems='center'
            justifyContent='center'
            pb={4}
          >
          <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleExecuteOption();}}>
                Confirm
          </Button>
          </PopoverFooter>
        </PopoverContent>
          </>
        )}
      </Popover>       
  )

}

const RemoveBidButton = ({
  id,
}: {
  id:number;
})=>{
  const initialFocusRef = React.useRef()
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [price, setPrice] = useState(0)
  const {tx} = useMarketTx(chainName,"unibi",MarketAddress)
  const { address, getCosmWasmClient } = useChain(chainName);

  const handleExecuteOption = async () => {
      setIsSubmitting(true)

      let msg:ExecuteMsg = {
        remove_bid: {
          id: id
        }
      }
      console.log(msg)
      await tx(msg,{});
      setIsSubmitting(false)
    }

    const handleQueryOwnerbid =async () =>{
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
      let bidsPromise = client.queryContractSmart(MarketAddress, {
        bid_list: {id:id}
    });
    
    bidsPromise.then(bids => {
        // 这里的 bids 是一个数组，你可以对其进行遍历操作
        for (let i = 0; i < bids.length; i++) {
            let bid:[Addr, Bid] = bids[i];
            console.log(bid)
            // 对每个元素进行操作
            if (bid[0] == address){
              setPrice(parseInt(bid[1].price))
              console.log("price is:",bid[1].price )
            }
        }
    }).catch(error => {
        // 处理可能的错误
        console.error("Error fetching bids:", error);
    });
    
    } 

  return (
      <Popover
        initialFocusRef={initialFocusRef}
        placement='bottom'
      >{({ isOpen, onClose }) => (
      <>
        <PopoverTrigger>
          <Button isLoading={isSubmitting} colorScheme="primary" onClick={handleQueryOwnerbid}>remove bid</Button>
        </PopoverTrigger>
        <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
          <PopoverHeader pt={4} fontWeight='bold' border='10' >
            Option ID: {id} 
          </PopoverHeader>
          <PopoverArrow bg='blue.800' />
          <PopoverCloseButton />
          <PopoverBody>
          </PopoverBody>
            Your bid price is :{price/1000000} NIBI
          <PopoverFooter
            border='0'
            display='flex'
            alignItems='center'
            justifyContent='center'
            pb={4}
          >
          <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleExecuteOption();}}>
                Confirm
          </Button>
          </PopoverFooter>
        </PopoverContent>
          </>
        )}
      </Popover>       
  )

}



const OptionCard = ({
    data,
    id
  }: {
    data: Data;
    id:number
  }) => {
    const {assets} = useChain(chainName)
    const getdenomMap =() => {
        let map = new Map<String,String>()
        assets.assets.map((value)=>{
            map.set(value.denom_units[0].denom,value.name)
        })
        return map;
    }

    return (
      <Box
        bg={useColorModeValue('gray.50', 'whiteAlpha.200')}
        borderRadius="xl"
        boxShadow={useColorModeValue('0 0 2px gray', '0 0 2px white')}
        p={6}
        w = "full"
      >
        <VStack align="start" fontWeight="bold" fontSize={{ md: 'lg' }} color={useColorModeValue('blackAlpha.700', 'whiteAlpha.700')} mb={1}>
           <Flex justify="space-between" w = "full"><Text flex={1} mr={2}> ID</Text><Text>{id}</Text></Flex> 
           <Flex justify="space-between" w = "full"><Text> Collateral:</Text>
            <Text>
              {address ? (
                `${Number(data.collateral.amount)/1000000} ${getdenomMap().get(data.collateral.denom)}`
              ) : (
                <Tooltip label="Connect wallet to see the value" placement="top">
                  <Text cursor="default">-</Text>
                </Tooltip>
              )}
            </Text>
           </Flex> 
           <Flex justify="space-between" w = "full"><Text> Counter offer:</Text>
            <Text>
              {address ? (
                `${Number(data.counter_offer.amount)/1000000} ${getdenomMap().get(data.counter_offer.denom)}`
              ) : (
                <Tooltip label="Connect wallet to see the value" placement="top">
                    <Text cursor="default">-</Text>
                </Tooltip>
              )}
            </Text>
           </Flex> 
           <Flex justify="space-between" w = "full"><Text> Expiration date: </Text><Text>{(new Date(Number(data.expires)/1000000)).toDateString()}</Text></Flex>
           <Flex justify="end" w = "full">
            <BidButton id={id}/>
            <Spacer></Spacer>
            <RemoveBidButton id={id}/>
           </Flex>
        </VStack>        
      </Box>
    );
  };
  


export const AllOptionList = ()=>{
    const { address, getCosmWasmClient } = useChain(chainName);
    const [datas, setData] = useState<ArrayOfTupleOfUint64AndData>() 

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
            options: {}
          })
        options.then((value)=>{setData(value);console.log(value)})
    } 
    return(
    <Box>
        <VStack spacing={5}>
        <Button onClick={handleQueryOwnerList} w="full" justifyContent="center" >Refresh options</Button>
        {datas?(
            datas.map((data)=>{
                return(
                        <OptionCard data={data[1]} id={data[0]} key = {data[0]}/>
                )
            })
        ):(<Skeleton w="full" h={{ base: 6, sm: 100 }} />)}
        </VStack>
    </Box>
    )
}
