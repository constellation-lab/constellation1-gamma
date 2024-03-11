import { chainName,MarketAddress } from "../config"
import { useChain } from '@cosmos-kit/react';
import { Box, Button ,Container,List,Skeleton, Stack,VStack,useColorModeValue,Flex,Text,Popover,PopoverTrigger,PopoverContent,PopoverHeader,PopoverArrow,PopoverCloseButton,
    PopoverFooter,PopoverBody,Input, HStack,Select,Editable,EditableInput,EditablePreview} from "@chakra-ui/react";
import { useState } from "react";
import React from "react";
import { ExecuteMsg,ListItemData,ArrayOfTupleOfUint64AndListItemData} from "../config/market";
import { Coin } from "@cosmjs/amino";
import { useMarketTx } from "../hook";
import { get } from "http";


const ListMarketButton = ({
    id,
    expires
  }: {
    id:number;
    expires: number;
  })=>{
    const initialFocusRef = React.useRef()
    const { assets, getCosmWasmClient } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [price, setPrice] = useState(0)
    const [priceToken,setPriceToken] = useState("unibi")
    const {tx} = useMarketTx(chainName,"unibi",MarketAddress)

    const handleExecuteOption = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {
            list:{
                expires:Math.floor((Date.now()+800*24*60*60*1000)/1000),
                price:(price*1000000).toString(),
                id:id,
            }
        }
        console.log(msg)
        await tx(msg,{});
        setIsSubmitting(false)
      }
  

    return (
        <Popover
          initialFocusRef={initialFocusRef}
          placement='bottom'
        >{({ isOpen, onClose }) => (
        <>
          <PopoverTrigger>
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>expires/1000000)}>List Market</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
              List Option ID: {id} 
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            Set Price:
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
           <Flex justify="space-between" w = "full"><Text> collateral:</Text><Text>{Number(data.collateral.amount)/1000000}{getdenomMap().get(data.collateral.denom)}</Text></Flex> 
           <Flex justify="space-between" w = "full"><Text> count offer:</Text><Text>{Number(data.counter_offer.amount)/1000000}{getdenomMap().get(data.counter_offer.denom)}</Text></Flex> 
           <Flex justify="space-between" w = "full"><Text> expiration date: </Text><Text>{(new Date(Number(data.expires)/1000000)).toDateString()}</Text></Flex>
           <Flex justify="space-between" w = "full">
            <ListMarketButton id = {id} expires={Number(data.expires)}/>
           </Flex>
        </VStack>        
      </Box>
    );
  };
  


export const ListOptionList = ()=>{
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
            "owner_un_list_items":{
             "addr":address
          }
          })
        options.then((value)=>{setData(value);console.log(value)})
    } 
    return(
    <Box>
        <VStack spacing={5}>
        <Button onClick={handleQueryOwnerList} w="full" justifyContent="center" >Refresh the options you owned</Button>
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