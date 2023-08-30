import { chainName,contractAddress } from "../config"
import { useChain } from '@cosmos-kit/react';
import { Box, Button ,Container,List,Skeleton, Stack,VStack,useColorModeValue,Flex,Text,Popover,PopoverTrigger,PopoverContent,PopoverHeader,PopoverArrow,PopoverCloseButton,
    PopoverFooter,PopoverBody,Input, HStack,Select,Editable,EditableInput,EditablePreview} from "@chakra-ui/react";
import { ArrayOfTupleOfUint64AndData } from "../config/constellation";
import { useState } from "react";
import { Data } from '../config/constellation/Constellation.types';
import React from "react";
import { ExecuteMsg } from "../config/constellation/Constellation.types";
import { Coin } from "@cosmjs/amino";
import { useTx } from "../hook";
import { get } from "http";

const TransferButton = ({
    id,
    expires
  }: {
    id:number;
    expires: number;
  })=>{
    const initialFocusRef = React.useRef()
    const { address, getCosmWasmClient } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [toAddr, setToAddr] = useState("")
    const {tx} = useTx(chainName,"unibi",contractAddress)

    const handleTransferOption = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {transfer:{
            id: id,
            to: toAddr
        }}
        console.log(msg)
        await tx(msg,{});
        setIsSubmitting(false)
      }
  

    const validateAddr =()=>{
        if (toAddr.length != 43){
            return true
        }
        const Regex =/^nibi/;
        const iserror = toAddr.match(Regex);
        if (iserror === null) {
            return true;
        }
        return false
    }
    return (
        <Popover
          initialFocusRef={initialFocusRef}
          placement='bottom'
        >{({ isOpen, onClose }) => (
        <>
          <PopoverTrigger>
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>expires/1000000)}>Transfer</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
              Option ID: {id} 
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            TRANSFER OPTION TO:
            <Input variant='flushed' isInvalid={validateAddr()} placeholder='Put Address here...' onChange={(input)=>{
                const value = input.target.value;
                setToAddr(value);
            }}/>
            {validateAddr()?(  <Text fontSize='xs' color={"tomato"}>invalid address</Text>):(<></>)}

            </PopoverBody>

            <PopoverFooter
              border='0'
              display='flex'
              alignItems='center'
              justifyContent='center'
              pb={4}
            >
            <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleTransferOption();}} isDisabled = {validateAddr()}>
                  Confirm
            </Button>
            </PopoverFooter>
          </PopoverContent>
            </>
          )}
        </Popover>       
    )

}

const ExeButton = ({
    id,
    data
  }: {
    id:number;
    data:Data
})=>{
    const initialFocusRef = React.useRef()
    const { address, assets } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [toAddr, setToAddr] = useState("")
    const {tx} = useTx(chainName,"unibi",contractAddress)
    const getdenomMap =() => {
        let map = new Map<String,String>()
        assets.assets.map((value)=>{
            map.set(value.denom_units[0].denom,value.name)
        })
        return map;
    }

    const handleExecuteOption = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {execute:{
            id:id
        }}
        const funds:Coin[]=data.counter_offer
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
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>Number(data.expires)/1000000)}>Execute</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
            YOU WILL EXECUTE OPTION: {id} 
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            <VStack align="start" fontWeight="bold" fontSize={{ md: 'lg' }}  mb={1}>
                <Flex justify="space-between" w = "full"><Text> you will get:</Text><Text>{Number(data.collateral[0].amount)/1000000}{getdenomMap().get(data.collateral[0].denom)}</Text></Flex> 
                <Flex justify="space-between" w = "full"><Text> you need to pay:</Text><Text>{Number(data.counter_offer[0].amount)/1000000}{getdenomMap().get(data.counter_offer[0].denom)}</Text></Flex> 
           </VStack>        

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
    const {tx} = useTx(chainName,"unibi",contractAddress)

    const handleExecuteOption = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {
            add_to_market:{
                amount:(price*1000000).toString(),
                denom:priceToken,
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
            <Select placeholder='Select price token' onChange={(value)=>{setPriceToken(value.target.value)}}>
                {assets.assets?(
                    assets.assets.map((asset)=>{return(<option value={asset.denom_units[0].denom} key={asset.name}>{asset.name}</option>)})
                ):(<></>)}
            </Select>
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

const UnlistButton = ({
    id,
    data
  }: {
    id:number;
    data:Data
})=>{
    const initialFocusRef = React.useRef()
    const { address, assets } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [toAddr, setToAddr] = useState("")
    const {tx} = useTx(chainName,"unibi",contractAddress)
    const getdenomMap =() => {
        let map = new Map<String,String>()
        assets.assets.map((value)=>{
            map.set(value.denom_units[0].denom,value.name)
        })
        return map;
    }

    const handleUnlistOption = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {remove_from_market:{
            id:id
        }}
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
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>Number(data.expires)/1000000)}>UnList</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
            YOU WILL EXECUTE OPTION: {id} 
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            <VStack align="start" fontWeight="bold" fontSize={{ md: 'lg' }}  mb={1}>
                <Flex justify="space-between" w = "full"><Text>List Price is:</Text><Text>{Number(data.price[0].amount)/1000000}{getdenomMap().get(data.price[0].denom)}</Text></Flex> 
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
    data:Data;
    expires: number;
  })=>{
    const initialFocusRef = React.useRef()
    const { assets, getCosmWasmClient } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [price, setPrice] = useState(0)
    const [priceToken,setPriceToken] = useState("unibi")
    const {tx} = useTx(chainName,"unibi",contractAddress)
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
                price:[{amount: (price*1000000).toString(),denom: priceToken}]
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
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>expires/1000000) || !data.onsale} >Udate Pice</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            {data.price[0]?( <PopoverHeader pt={4} fontWeight='bold' border='10' >
              current price is: {Number(data.price[0].amount)/1000000} {getdenomMap().get(data.price[0].denom)}
            </PopoverHeader>):(<PopoverHeader pt={4} fontWeight='bold' border='10' >current price is not set</PopoverHeader>)}
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
            <Select placeholder='Select price token' onChange={(value)=>{setPriceToken(value.target.value)}}>
                {assets.assets?(
                    assets.assets.map((asset)=>{return(<option value={asset.denom_units[0].denom} key={asset.name}>{asset.name}</option>)})
                ):(<></>)}
            </Select>
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
           <Flex justify="space-between" w = "full"><Text> collateral:</Text><Text>{Number(data.collateral[0].amount)/1000000}{getdenomMap().get(data.collateral[0].denom)}</Text></Flex> 
           <Flex justify="space-between" w = "full"><Text> count offer:</Text><Text>{Number(data.counter_offer[0].amount)/1000000}{getdenomMap().get(data.counter_offer[0].denom)}</Text></Flex> 
           <Flex justify="space-between" w = "full"><Text> expiration date: </Text><Text>{(new Date(Number(data.expires)/1000000)).toDateString()}</Text></Flex>
           <Flex justify="space-between" w = "full">
            <TransferButton id={id} expires={Number(data.expires)}/>
            <ExeButton id={id} data={data}/>
            {data.onsale?(<UnlistButton id = {id} data= {data}/>):(<ListMarketButton id = {id} expires={Number(data.expires)}/>)}
            {data.onsale?(<UpdatePriceButton id={id} data={data} expires ={Number(data.expires)}/>):(<></>)}
           </Flex>
        </VStack>        
      </Box>
    );
  };
  


export const OwnerOptionList = ()=>{
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
            "owner_options":{
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