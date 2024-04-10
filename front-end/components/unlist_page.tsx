import { chainName,MarketAddress } from "../config"
import { useChain } from '@cosmos-kit/react';
import { Box, Button ,Container,List,Skeleton, Stack,VStack,useColorModeValue,Flex,Text,Popover,PopoverTrigger,PopoverContent,PopoverHeader,PopoverArrow,PopoverCloseButton,
    PopoverFooter,PopoverBody,Input, HStack,Select,Editable,EditableInput,EditablePreview, Tooltip} from "@chakra-ui/react";
import { ArrayOfTupleOfUint64AndListItemData,ListItemData,ExecuteMsg } from "../config/market";
import { useState } from "react";
import React from "react";
import { Coin } from "@cosmjs/amino";
import { useMarketTx } from "../hook";
import { get } from "http";




const UnlistButton = ({
    id,
    data
  }: {
    id:number;
    data:ListItemData
})=>{
    const initialFocusRef = React.useRef()
    const { address, assets } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [toAddr, setToAddr] = useState("")
    const {tx} = useMarketTx(chainName,"unibi",MarketAddress)
    const getdenomMap =() => {
        let map = new Map<String,String>()
        assets.assets.map((value)=>{
            map.set(value.denom_units[0].denom,value.name)
        })
        return map;
    }

    const handleUnlistOption = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {
            remove_list: {
                id: id
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
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>Number(data.expires)/1000000)}>Unlist</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
            You are about to unlist the option with id: {id} 
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            <VStack align="start" fontWeight="bold" fontSize={{ md: 'lg' }}  mb={1}>
                <Flex justify="space-between" w = "full"><Text>Listed price is:</Text>
                <Text>
                  {address ? (
                    `${Number(data.price)/1000000} NIBI`
                  ) : (
                  <Tooltip label="Connect wallet to see the value" placement="top">
                      <Text cursor="default">-</Text>
                  </Tooltip>
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
            <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleUnlistOption();}}>
                  Confirm
            </Button>
            </PopoverFooter>
          </PopoverContent>
            </>
          )}
        </Popover>       
    )

}

const UpdatePriceButton = ({
    id,
    data,
    expires
  }: {
    id:number;
    data:ListItemData;
    expires: number;
  })=>{
    const initialFocusRef = React.useRef()
    const { assets, getCosmWasmClient } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [price, setPrice] = useState(0)
    const [priceToken,setPriceToken] = useState("unibi")
    const {tx} = useMarketTx(chainName,"unibi",MarketAddress)
    const getdenomMap =() => {
        let map = new Map<String,String>()
        assets.assets.map((value)=>{
            map.set(value.denom_units[0].denom,value.name)
        })
        return map;
    }


    const handleUpdatePrice = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {
            update_price:{
                id:id,
                price:(price*1000000).toString()
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
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>expires/1000000)} >Update Price</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            {data.price[0]?( <PopoverHeader pt={4} fontWeight='bold' border='10' >
              The Current price is: {Number(data.price)/1000000} NIBI
            </PopoverHeader>):(<PopoverHeader pt={4} fontWeight='bold' border='10' >The Current price is not set</PopoverHeader>)}
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
            <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleUpdatePrice();}}>
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
    const {assets, address} = useChain(chainName)
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
           <Flex justify="space-between" w = "full"><Text> Expiration Date: </Text><Text>{(new Date(Number(data.expires)/1000000)).toDateString()}</Text></Flex>
           <Flex justify="space-between" w = "full">
            <UnlistButton id = {id} data= {data}/>
            <UpdatePriceButton id={id} data={data} expires ={Number(data.expires)}/>
           </Flex>
        </VStack>        
      </Box>
    );
  };
  


export const ManageOptionList = ()=>{
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
            "owner_list_items":{
             "addr":address
          }
          })
        options.then((value)=>{setData(value);console.log(value)})
    } 
    return(
    <Box>
        <VStack spacing={5}>
        <Button onClick={handleQueryOwnerList} w="full" justifyContent="center" >Refresh the options you have listed</Button>
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
