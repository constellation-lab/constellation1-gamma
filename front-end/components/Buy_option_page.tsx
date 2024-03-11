import { chainName,MarketAddress } from "../config"
import { useChain } from '@cosmos-kit/react';
import { Box, Button,Skeleton,VStack,useColorModeValue,Flex,Text,Popover,PopoverTrigger,PopoverContent,PopoverHeader,PopoverArrow,PopoverCloseButton,
    PopoverFooter,PopoverBody} from "@chakra-ui/react";
import{ArrayOfTupleOfUint64AndListItemData,ListItemData,ExecuteMsg} from "../config/market"
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
    const { assets } = useChain(chainName);
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
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>Number(data.expires)/1000000) } size='lg'>Buy It</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
            YOU WILL Buy OPTION: {id} 
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            <VStack align="start" fontWeight="bold" fontSize={{ md: 'lg' }}  mb={1}>
                <Flex justify="space-between" w = "full"><Text> you need pay:</Text><Text>{Number(data.price)/1000000}NIBI</Text></Flex> 
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


const OptionCard = ({
    data,
    id
  }: {
    data: ListItemData;
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
           <Flex justify="space-between" w = "full"><Text> creator is: </Text><Text>{data.creator}</Text></Flex>
           <Flex justify="space-between" w = "full"><Text> owner is: </Text><Text>{data.owner}</Text></Flex>

           <Flex justify="space-between" w = "full"><Text> collateral:</Text><Text>{Number(data.collateral.amount)/1000000}{getdenomMap().get(data.collateral.denom)}</Text></Flex> 
           <Flex justify="space-between" w = "full"><Text> count offer:</Text><Text>{Number(data.counter_offer.amount)/1000000}{getdenomMap().get(data.counter_offer.denom)}</Text></Flex> 
           <Flex justify="space-between" w = "full"><Text> expiration date: </Text><Text>{(new Date(Number(data.expires)/1000000)).toDateString()}</Text></Flex>

           <Flex justify="space-between" w = "full" fontSize='5xl' color = "blue.500"><Text> price is: </Text><Text>{Number(data.price)/1000000}NIBI</Text></Flex>
           <Flex justify="end" w = "full">
            <BuyButton id={id} data={data}/>
           </Flex>
        </VStack>        
      </Box>
    );
  };
  


export const BuyOptionsList = ()=>{
    const { address, getCosmWasmClient } = useChain(chainName);
    const [datas, setData] = useState<ArrayOfTupleOfUint64AndListItemData>() 

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
        let options:Promise<ArrayOfTupleOfUint64AndListItemData>  = client.queryContractSmart(MarketAddress,{
            "list_items":{}
          })
        options.then((value)=>{setData(value);console.log(value)})
    } 
    return(
    <Box>
        <VStack spacing={5}>
        <Button onClick={handleQueryOwnerList} w="full" justifyContent="center" >Refresh the options in market</Button>
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