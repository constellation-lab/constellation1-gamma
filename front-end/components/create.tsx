import {
  Box,
  Button,
  Collapse,
  Divider,
  Editable,
  EditableInput,
  EditablePreview,
  Flex,
  Grid,
  Icon,
  Image,
  Popover,
  PopoverBody,
  PopoverContent,
  PopoverTrigger,
  Skeleton,
  Stack,
  SystemStyleObject,
  Text,
  useColorModeValue,
  useDisclosure,
  useOutsideClick,
  useRadio,
  useRadioGroup
} from '@chakra-ui/react';
import { chains } from 'chain-registry';
import {
  AsyncSelect,
  chakraComponents,
  ControlProps,
  GroupBase,
  OptionBase,
  OptionProps
} from 'chakra-react-select';
import React, { useEffect, useRef, useState } from 'react';
import {
  BsExclamationCircleFill,
  BsHexagon,
  BsHexagonFill
} from 'react-icons/bs';
import { CgArrowsExchangeV } from 'react-icons/cg';
import { FiChevronDown, FiChevronUp } from 'react-icons/fi';
import { RiSearch2Fill, RiSettings4Fill } from 'react-icons/ri';
import { useChain } from '@cosmos-kit/react';
import { chainName } from '../config';
import { type } from 'os';

interface dataType extends OptionBase {
  label: string;
  value: string;
  denom:string;
  imgSrc?: string;
  ibc?: {
    source_channel?: string;
    dst_channel?: string;
    source_denom?: string;
  };
}

const RadioTag = (props: any) => {
  const { getInputProps, getCheckboxProps } = useRadio(props);
  const input = getInputProps();
  const checkbox = getCheckboxProps();

  return (
    <Box as="label">
      <input {...input} />
      <Box
        {...checkbox}
        cursor="pointer"
        bg={useColorModeValue('blackAlpha.300', 'whiteAlpha.300')}
        borderRadius="full"
        _checked={{
          bg: 'primary.500',
          color: 'white'
        }}
        _focus={{
          boxShadow: 'outline'
        }}
        _disabled={{
          cursor: 'not-allowed',
          opacity: 0.5
        }}
        px={5}
        py={1}
      >
        <Text textAlign="center">{props.children}</Text>
      </Box>
    </Box>
  );
};

const Setting = () => {
  const { onToggle, onClose, isOpen } = useDisclosure();
  const initialFocusRef = useRef(null);
  const options = ['1%', '3%', '5%', '2.5%'];
  const { getRootProps, getRadioProps } = useRadioGroup({
    name: 'setting',
    defaultValue: '1%',
    onChange: console.log
  });
  const group = getRootProps();

  return (
    <Popover
      isOpen={isOpen}
      onClose={onClose}
      initialFocusRef={initialFocusRef}
    >
      <PopoverTrigger>
        <Button
          position="relative"
          variant="unstyled"
          w="min"
          h="min"
          color={
            isOpen
              ? 'orange.300'
              // eslint-disable-next-line react-hooks/rules-of-hooks
              : useColorModeValue('blackAlpha.400', 'whiteAlpha.500')
          }
          transition="all .5s"
          _hover={{
            color: isOpen
              ? 'orange.200'
              // eslint-disable-next-line react-hooks/rules-of-hooks
              : useColorModeValue('blackAlpha.500', 'whiteAlpha.600')
          }}
          _focus={{ boxShadow: 'none' }}
          onClick={onToggle}
        >
          <Icon
            zIndex={-10}
            as={BsHexagonFill}
            w={8}
            h={8}
            color={useColorModeValue('gray.100', 'whiteAlpha.300')}
          />
          <Icon
            position="absolute"
            top={0}
            left={1}
            right={0}
            zIndex={10}
            as={BsHexagon}
            w={8}
            h={8}
          />
          <Icon
            position="absolute"
            top={2}
            left={3}
            right={2}
            w={4}
            h={4}
            as={RiSettings4Fill}
          />
        </Button>
      </PopoverTrigger>
      <PopoverContent
        bg={useColorModeValue('white', 'black')}
        borderColor={useColorModeValue('blackAlpha.200', 'whiteAlpha.400')}
        boxShadow="md"
        w="fit-content"
        right={4}
      >
        <PopoverBody p={{ base: 6, sm: 8 }}>
          <Text fontWeight="semibold" mb={1.5}>
            Transaction Setting
          </Text>
          <Text
            fontWeight="semibold"
            color={useColorModeValue('blackAlpha.500', 'whiteAlpha.600')}
            verticalAlign="middle"
            mb={4}
          >
            Slippage tolerance&ensp;
            <Icon as={BsExclamationCircleFill} color="orange.200" />
          </Text>
          <Grid
            templateColumns={{ base: '1fr 1fr', sm: 'repeat(4, 1fr)' }}
            gap={4}
            {...group}
          >
            {options.map((value) => {
              const radio = getRadioProps({ value });
              return (
                <RadioTag
                  key={value}
                  value={value}
                  isDisabled={value === '2.5%' ? true : false}
                  {...radio}
                >
                  {value}
                </RadioTag>
              );
            })}
          </Grid>
        </PopoverBody>
      </PopoverContent>
    </Popover>
  );
};

const SkeletonOptions = () => {
  return (
    <>
      <Flex justify="space-between" align="center" mb={{ base: 2, sm: 4 }}>
        <Flex align="center">
          <Skeleton
            w={{ base: 10, sm: 16 }}
            h={{ base: 10, sm: 16 }}
            mr={{ base: 2, sm: 4 }}
          />
          <Skeleton w={{ base: 24, sm: 48 }} h={{ base: 6, sm: 8 }} />
        </Flex>
        <Skeleton w={{ base: 24, sm: 48 }} h={{ base: 6, sm: 8 }} />
      </Flex>
      <Flex justify="space-between" align="center" mb={{ base: 2, sm: 4 }}>
        <Flex align="center">
          <Skeleton
            w={{ base: 10, sm: 16 }}
            h={{ base: 10, sm: 16 }}
            mr={{ base: 2, sm: 4 }}
          />
          <Skeleton w={{ base: 24, sm: 48 }} h={{ base: 6, sm: 8 }} />
        </Flex>
        <Skeleton w={{ base: 24, sm: 48 }} h={{ base: 6, sm: 8 }} />
      </Flex>
      <Flex justify="space-between" align="center">
        <Flex align="center">
          <Skeleton
            w={{ base: 10, sm: 16 }}
            h={{ base: 10, sm: 16 }}
            mr={{ base: 2, sm: 4 }}
          />
          <Skeleton w={{ base: 24, sm: 48 }} h={{ base: 6, sm: 8 }} />
        </Flex>
        <Skeleton w={{ base: 24, sm: 48 }} h={{ base: 6, sm: 8 }} />
      </Flex>
    </>
  );
};

const FromToken = ({
  data,
  fromItem,
  setFromItem,
  toItem,
  setToItem,
  tokenInputValue,
  setTokenInputValue
}: {
  data: dataType[];
  fromItem: dataType | undefined;
  setFromItem: (value: dataType) => void;
  toItem: dataType | undefined;
  setToItem: (value: dataType) => void;
  tokenInputValue: number;
  setTokenInputValue: (value: number) => void;
}) => {
  const [checked, setChecked] = useState([false, false]);
  const [checkedItems, setCheckedItems] = useState([
    {
      label: 'MAX',
      id: 'max',
      lightBg: 'blackAlpha.300',
      darkBg: 'whiteAlpha.300'
    },
    {
      label: 'HALF',
      id: 'half',
      lightBg: 'blackAlpha.300',
      darkBg: 'whiteAlpha.300'
    }
  ]);
  const [collateral, setCollateral] = useState<number>();
  const fromMenuRef = useRef<HTMLDivElement | null>(null);
  const { isOpen, onToggle, onClose } = useDisclosure();
  const {getStargateClient,address} = useChain(chainName);

  const customStyles = {
    control: (provided: SystemStyleObject) => ({
      ...provided,
      // eslint-disable-next-line react-hooks/rules-of-hooks
      bg: useColorModeValue('blackAlpha.50', 'whiteAlpha.50')
    }),
    menu: (provided: SystemStyleObject) => ({
      ...provided,
      maxH: { base: 'sm', sm: '2xl' },
      position: 'relative',
      mt: 6,
      mb: 0
    }),
    menuList: (provided: SystemStyleObject) => ({
      ...provided,
      bg: 'transparent',
      border: 'none',
      borderRadius: 'none',
      py: 0,
      pr: { base: 2, sm: 4 },
      // For Firefox
      scrollbarWidth: 'auto',
      // eslint-disable-next-line react-hooks/rules-of-hooks
      scrollbarColor: useColorModeValue(
        'rgba(0,0,0,0.3) rgba(0,0,0,0.2)',
        'rgba(255,255,255,0.2) rgba(255,255,255,0.1)'
      ),
      // For Chrome and other browsers except Firefox
      '&::-webkit-scrollbar': {
        width: '18px',
        // eslint-disable-next-line react-hooks/rules-of-hooks
        background: useColorModeValue(
          'rgba(160,160,160,0.1)',
          'rgba(255,255,255,0.1)'
        ),
        borderRadius: '4px'
      },
      '&::-webkit-scrollbar-thumb': {
        // eslint-disable-next-line react-hooks/rules-of-hooks
        background: useColorModeValue(
          'rgba(0,0,0,0.1)',
          'rgba(255,255,255,0.1)'
        ),
        borderRadius: '4px'
      }
    }),
    option: (provided: SystemStyleObject, state: { isSelected: boolean }) => ({
      ...provided,
      borderRadius: 'lg',
      bg: state.isSelected
        // eslint-disable-next-line react-hooks/rules-of-hooks
        ? useColorModeValue('primary.100', 'primary.500')
        : 'transparent',
      color: 'inherit',
      _hover: {
        bg: state.isSelected
          // eslint-disable-next-line react-hooks/rules-of-hooks
          ? useColorModeValue('primary.100', 'primary.500')
          // eslint-disable-next-line react-hooks/rules-of-hooks
          : useColorModeValue('blackAlpha.200', 'whiteAlpha.200')
      },
      _disabled: {
        _hover: { bg: 'transparent' }
      }
    })
  };
  const IndicatorSeparator = () => {
    return null;
  };
  const DropdownIndicator = () => {
    return null;
  };
  
  const CustomOption = ({
    children,
    ...props
  }: OptionProps<dataType, true, GroupBase<dataType>>) => {
    return (
      <chakraComponents.Option {...props}>
        <Flex id={props.data.value} align="center" w="full">
          <Flex align="center" flex={1} mr={2}>
            <Box
              minW={{ base: 12, sm: 16 }}
              minH={{ base: 12, sm: 16 }}
              maxW={{ base: 12, sm: 16 }}
              maxH={{ base: 12, sm: 16 }}
              w="full"
              h="full"
              mr={{ base: 3, sm: 4 }}
            >
              <Image src={props.data.imgSrc} />
            </Box>
            <Box>
              <Text
                fontSize={{ base: 'lg', sm: '2xl' }}
                fontWeight="bold"
                textAlign="start"
              >
                {children}
              </Text>
              <Text
                fontSize={{ base: 'md', sm: 'lg' }}
                fontWeight="bold"
                textAlign="start"
                color={useColorModeValue('blackAlpha.700', 'whiteAlpha.700')}
              >
                {props.data.ibc?.source_channel}
              </Text>
            </Box>
          </Flex>
          <Text
            fontSize={{ base: 'md', sm: 'xl' }}
            fontWeight="semibold"
            textAlign="end"
            wordBreak="break-word"
          >
            3.141595
          </Text>
        </Flex>
      </chakraComponents.Option>
    );
  };
  const CustomControl = ({
    children,
    ...props
  }: ControlProps<dataType, true>) => {
    return (
      <chakraComponents.Control {...props}>
        <Flex align="center" pl={4}>
          <Icon as={RiSearch2Fill} />
        </Flex>
        {children}
      </chakraComponents.Control>
    );
  };
  const AvailableCheckbox = ({
    label,
    id,
    lightBg,
    darkBg,
    index
  }: {
    label: string;
    id: string;
    lightBg: string;
    darkBg: string;
    index: number;
  }) => {
    return (
      <Button
        id={id}
        variant="unstyled"
        fontSize="xs"
        bg={useColorModeValue(lightBg, darkBg)}
        color={useColorModeValue('blackAlpha.700', 'whiteAlpha.700')}
        borderRadius="md"
        fontWeight="semibold"
        onClick={(e) => {
          if (e.currentTarget.id === id) {
            setChecked((pre) => {
              const newArr = pre.map((v, i) => {
                if (i === index) return !v;
                return false;
              });
              return newArr;
            });
          }
        }}
        h={7}
        w={12}
      >
        {label}
      </Button>
    );
  };

  useEffect(() => {
    setCheckedItems((pre) => {
      const newItems = pre.map(({ lightBg, darkBg, ...rest }, i) => ({
        ...rest,
        lightBg: checked[i] ? 'primary.300' : 'blackAlpha.300',
        darkBg: checked[i] ? 'primary.800' : 'whiteAlpha.300'
      }));
      return newItems;
    });
    getStargateClient().then((client)=>{
        client.getBalance(address,fromItem.denom).then((coin)=>{
        setCollateral(Number(coin.amount))
      }).catch((error)=>{
        console.log(error)
      });
    }).catch((error)=>{
      console.log(error)
    })
  }, [checked]);
  useOutsideClick({
    ref: fromMenuRef,
    handler: () => onClose()
  });

  return (
    <Box
      ref={fromMenuRef}
      position="relative"
      bg={useColorModeValue('gray.100', 'gray.700')}
      borderRadius="xl"
      boxShadow={isOpen ? '0 0 20px -8px rgba(105, 88, 164, 0.5)' : 'none'}
      p={6}
    >
      <Flex
        position="relative"
        justify="space-between"
        flexDirection={{ base: 'column', sm: 'row' }}
        align={{ base: 'start', sm: 'center' }}
        mb={4}
      >
        <Text fontSize={{ base: 'md', sm: 'lg' }} fontWeight="bold">
        collateral
        </Text>
        <Flex
          maxW={{ sm: '2xs' }}
          w="full"
          justify="space-between"
          align="center"
        >
          <Text fontSize={{ base: 'md', sm: 'lg' }} fontWeight="bold">
            Available  
          </Text>
          <Text
            fontSize={{ base: 'md', sm: 'lg' }}
            fontWeight="bold"
            color="primary.300"
          >
            {(collateral/1000000).toFixed(2)}
          </Text>
          {checkedItems.map(({ label, id, lightBg, darkBg }, index) => (
            <AvailableCheckbox
              label={label}
              id={id}
              lightBg={lightBg}
              darkBg={darkBg}
              index={index}
              key = {index}
            />
          ))}
        </Flex>
      </Flex>
      <Flex align="center" maxW="full" h="fit-content">
        <Button
          flex={2}
          variant="unstyled"
          w="fit-content"
          h="fit-content"
          whiteSpace="normal"
          _focus={{ boxShadow: 'none' }}
          onClick={onToggle}
          mr={2}
        >
          {fromItem ? (
            <Flex align="center">
              <Box
                minW={{ base: 12, sm: 20 }}
                minH={{ base: 12, sm: 20 }}
                maxW={{ base: 12, sm: 20 }}
                maxH={{ base: 12, sm: 20 }}
                w="full"
                h="full"
                borderRadius="full"
                border="2px solid"
                borderColor="orange.300"
                mr={{ base: 2, sm: 4 }}
              >
                <Image  boxSize='75px' src={fromItem.imgSrc} />
              </Box>
              <Text
                fontSize={{ base: 'xl', sm: '3xl' }}
                fontWeight="bold"
                textAlign="start"
              >
                {fromItem.label}&nbsp;
              </Text>
              <Icon
                as={isOpen ? FiChevronUp : FiChevronDown}
                fontSize={{ base: 'xl', sm: '3xl' }}
                // eslint-disable-next-line react-hooks/rules-of-hooks
                color={useColorModeValue('blackAlpha.700', 'whiteAlpha.700')}
              />
            </Flex>
          ) : (
            <Flex align="center">
              <Skeleton
                w={{ base: 12, sm: 20 }}
                h={{ base: 12, sm: 20 }}
                mr={{ base: 2, sm: 4 }}
              />
              <Skeleton
                w={{ base: 24, sm: 48 }}
                h={{ base: 6, sm: 10 }}
                mr={{ base: 2, sm: 4 }}
              />
            </Flex>
          )}
        </Button>
        {fromItem ? (
          <Box flex={1}>
            <Editable
              variant="unstyled"
              fontSize={{ base: 'lg', sm: '2xl' }}
              fontWeight="bold"
              textAlign="end"
              mb={{ base: 1, sm: 2 }}
              placeholder="0"
            >
              <EditablePreview />
              <EditableInput
                type="number"
                min="0"
                defaultValue="0"
                onChange={(e) => {
                }}
                _focus={{ boxShadow: 'none' }}
              />
            </Editable>
            <Text
              fontSize={{ sm: 'xl' }}
              textAlign="end"
              fontWeight="bold"
              mb={0}
            >
              ≈$0
            </Text>
          </Box>
        ) : (
          <Flex flexDirection="column" align="end">
            <Skeleton w={{ base: 20, sm: 36 }} h={{ base: 8, sm: 10 }} mb={2} />
            <Skeleton w={{ base: 12, sm: 16 }} h={{ base: 6, sm: 8 }} />
          </Flex>
        )}
      </Flex>
      <Box
        position="absolute"
        zIndex={2000}
        bg={useColorModeValue('gray.100', 'gray.700')}
        boxShadow={isOpen ? '0 12px 20px -8px rgba(105, 88, 164, 0.5)' : 'none'}
        borderRadius="xl"
        left={0}
        right={0}
        px={6}
      >
        <Collapse in={isOpen} animateOpacity>
          <Box py={6}>
            {fromItem ? (
              <AsyncSelect
                placeholder="Search"
                chakraStyles={customStyles}
                isClearable={false}
                // isOptionDisabled={(option) => option.label === 'Ion'} // test option disabled
                blurInputOnSelect={true}
                controlShouldRenderValue={false}
                menuIsOpen={true}
                loadingMessage={() => <SkeletonOptions />}
                defaultOptions={data}
                value={fromItem}
                loadOptions={(inputValue, callback) => {
                  setTimeout(() => {
                    const values = data.filter((option) =>
                      option.label
                        .toLowerCase()
                        .includes(inputValue.toLowerCase())
                    );
                    callback(values);
                  }, 1000);
                }}
                onChange={(selectedOption) => {
                  let value = {};
                  value = { ...selectedOption };
                  setFromItem(value as dataType);
                  onClose();
                }}
                components={{
                  DropdownIndicator,
                  IndicatorSeparator,
                  Control: CustomControl,
                  Option: CustomOption
                }}
              />
            ) : (
              <SkeletonOptions />
            )}
          </Box>
        </Collapse>
      </Box>
      {!isOpen && (
        <Flex justify="center" align="center" boxShadow="base">
          <Box
            as="button"
            position="absolute"
            zIndex={5}
            bottom={{ base: -8, sm: -10 }}
            // eslint-disable-next-line react-hooks/rules-of-hooks
            color={useColorModeValue('blackAlpha.300', 'whiteAlpha.600')}
            onClick={() => {
              setFromItem(toItem as dataType);
              setToItem(fromItem as dataType);
            }}
          >
            <Icon
              zIndex={-10}
              as={BsHexagonFill}
              w={{ base: 12, sm: 16 }}
              h={{ base: 12, sm: 16 }}
              // eslint-disable-next-line react-hooks/rules-of-hooks
              color={useColorModeValue('gray.100', 'gray.700')}
            />
            <Icon
              position="absolute"
              top={0}
              left={0}
              right={0}
              zIndex={10}
              as={BsHexagon}
              w={{ base: 12, sm: 16 }}
              h={{ base: 12, sm: 16 }}
            />
            <Icon
              position="absolute"
              top={2}
              left={2}
              right={2}
              w={{ base: 8, sm: 12 }}
              h={{ base: 8, sm: 12 }}
              as={CgArrowsExchangeV}
            />
          </Box>
        </Flex>
      )}
    </Box>
  );
};

const ToToken = ({
  data,
  toItem,
  setToItem
}: {
  data: dataType[];
  toItem: dataType | undefined;
  setToItem: (value: dataType) => void;
}) => {
  const toMenuRef = useRef<HTMLDivElement | null>(null);
  const { isOpen, onToggle, onClose } = useDisclosure();
  const customStyles = {
    control: (provided: SystemStyleObject) => ({
      ...provided,
      // eslint-disable-next-line react-hooks/rules-of-hooks
      bg: useColorModeValue('blackAlpha.50', 'whiteAlpha.50')
    }),
    menu: (provided: SystemStyleObject) => ({
      ...provided,
      maxH: { base: 'sm', sm: '2xl' },
      position: 'relative',
      mt: 6,
      mb: 0
    }),
    menuList: (provided: SystemStyleObject) => ({
      ...provided,
      bg: 'transparent',
      border: 'none',
      borderRadius: 'none',
      py: 0,
      pr: { base: 2, sm: 4 },
      // For Firefox
      scrollbarWidth: 'auto',
      // eslint-disable-next-line react-hooks/rules-of-hooks
      scrollbarColor: useColorModeValue(
        'rgba(0,0,0,0.3) rgba(0,0,0,0.2)',
        'rgba(255,255,255,0.2) rgba(255,255,255,0.1)'
      ),
      // For Chrome and other browsers except Firefox
      '&::-webkit-scrollbar': {
        width: '18px',
        // eslint-disable-next-line react-hooks/rules-of-hooks
        background: useColorModeValue(
          'rgba(160,160,160,0.1)',
          'rgba(255,255,255,0.1)'
        ),
        borderRadius: '4px'
      },
      '&::-webkit-scrollbar-thumb': {
        // eslint-disable-next-line react-hooks/rules-of-hooks
        background: useColorModeValue(
          'rgba(0,0,0,0.1)',
          'rgba(255,255,255,0.1)'
        ),
        borderRadius: '4px'
      }
    }),
    option: (provided: SystemStyleObject, state: { isSelected: boolean }) => ({
      ...provided,
      borderRadius: 'lg',
      bg: state.isSelected
      // eslint-disable-next-line react-hooks/rules-of-hooks
        ? useColorModeValue('primary.100', 'primary.500')
        : 'transparent',
      color: 'inherit',
      _hover: {
        bg: state.isSelected
        // eslint-disable-next-line react-hooks/rules-of-hooks
          ? useColorModeValue('primary.100', 'primary.500')
          // eslint-disable-next-line react-hooks/rules-of-hooks
          : useColorModeValue('blackAlpha.200', 'whiteAlpha.200')
      },
      _disabled: {
        _hover: { bg: 'transparent' }
      }
    })
  };
  const IndicatorSeparator = () => {
    return null;
  };
  const DropdownIndicator = () => {
    return null;
  };
  const CustomOption = ({
    children,
    ...props
  }: OptionProps<dataType, true, GroupBase<dataType>>) => {
    return (
      <chakraComponents.Option {...props}>
        <Flex id={props.data.value} align="center" w="full">
          <Flex align="center" flex={1} mr={2}>
            <Box
              minW={{ base: 12, sm: 16 }}
              minH={{ base: 12, sm: 16 }}
              maxW={{ base: 12, sm: 16 }}
              maxH={{ base: 12, sm: 16 }}
              w="full"
              h="full"
              mr={{ base: 3, sm: 4 }}
            >
              <Image  src={props.data.imgSrc} />
            </Box>
            <Box>
              <Text
                fontSize={{ base: 'lg', sm: '2xl' }}
                fontWeight="bold"
                textAlign="start"
              >
                {children}
              </Text>
              <Text
                fontSize={{ base: 'md', sm: 'lg' }}
                fontWeight="bold"
                textAlign="start"
                color={useColorModeValue('blackAlpha.700', 'whiteAlpha.700')}
              >
                {props.data.ibc?.source_channel}
              </Text>
            </Box>
          </Flex>
          <Text
            fontSize={{ base: 'md', sm: 'xl' }}
            fontWeight="semibold"
            textAlign="end"
            wordBreak="break-word"
          >
            3.141595
          </Text>
        </Flex>
      </chakraComponents.Option>
    );
  };
  const CustomControl = ({
    children,
    ...props
  }: ControlProps<dataType, true>) => {
    return (
      <chakraComponents.Control {...props}>
        <Flex align="center" pl={4}>
          <Icon as={RiSearch2Fill} />
        </Flex>
        {children}
      </chakraComponents.Control>
    );
  };

  useOutsideClick({
    ref: toMenuRef,
    handler: () => onClose()
  });

  return (
    <Box
      ref={toMenuRef}
      position="relative"
      bg={useColorModeValue('gray.100', 'gray.700')}
      boxShadow={isOpen ? '0 0 20px -8px rgba(105, 88, 164, 0.5)' : 'none'}
      borderRadius="xl"
      p={6}
    >
      <Text fontSize={{ base: 'md', sm: 'lg' }} fontWeight="bold" mb={4}>
        To
      </Text>
      <Flex align="center" maxW="full" h="fit-content">
        <Button
          flex={1}
          variant="unstyled"
          w="fit-content"
          h="fit-content"
          whiteSpace="normal"
          _focus={{ boxShadow: 'none' }}
          onClick={onToggle}
          mr={2}
        >
          {toItem ? (
            <Flex align="center">
              <Box
                minW={{ base: 12, sm: 20 }}
                minH={{ base: 12, sm: 20 }}
                maxW={{ base: 12, sm: 20 }}
                maxH={{ base: 12, sm: 20 }}
                w="full"
                h="full"
                borderRadius="full"
                border="2px solid"
                borderColor="orange.200"
                mr={{ base: 2, sm: 4 }}
              >
                <Image boxSize='75px' src={toItem.imgSrc} />
              </Box>
              <Text
                fontSize={{ base: 'xl', sm: '3xl' }}
                fontWeight="bold"
                textAlign="start"
              >
                {toItem.label}&nbsp;
              </Text>
              <Icon
                as={isOpen ? FiChevronUp : FiChevronDown}
                fontSize={{ base: 'xl', sm: '3xl' }}
                // eslint-disable-next-line react-hooks/rules-of-hooks
                color={useColorModeValue('blackAlpha.700', 'whiteAlpha.700')}
              />
            </Flex>
          ) : (
            <Flex align="center">
              <Skeleton
                w={{ base: 12, sm: 20 }}
                h={{ base: 12, sm: 20 }}
                mr={{ base: 2, sm: 4 }}
              />
              <Skeleton
                w={{ base: 24, sm: 48 }}
                h={{ base: 6, sm: 10 }}
                mr={{ base: 2, sm: 4 }}
              />
            </Flex>
          )}
        </Button>
        {toItem ? (
          <Flex
            maxW={{ base: 28, sm: 48, md: 'initial' }}
            wrap="wrap"
            justify="end"
          >
            <Text
              fontSize={{ base: 'lg', sm: '2xl', md: '3xl' }}
              fontWeight="bold"
              // eslint-disable-next-line react-hooks/rules-of-hooks
              color={useColorModeValue('blackAlpha.800', 'whiteAlpha.800')}
              textAlign="end"
              wordBreak="break-word"
              mb={{ base: 1, sm: 2 }}
              mr={{ base: 0, md: 2 }}
            >
              ≈&nbsp;3.265358
            </Text>
            <Text
              fontSize={{ base: 'lg', sm: '2xl', md: '3xl' }}
              fontWeight="bold"
              textAlign="end"
              // eslint-disable-next-line react-hooks/rules-of-hooks
              color={useColorModeValue('blackAlpha.800', 'whiteAlpha.800')}
            >
              {toItem.label}
            </Text>
          </Flex>
        ) : (
          <Skeleton w={{ base: 20, sm: 36 }} h={{ base: 8, sm: 10 }} />
        )}
      </Flex>
      <Box
        position="absolute"
        zIndex="dropdown"
        bg={useColorModeValue('gray.100', 'gray.700')}
        boxShadow={isOpen ? '0 12px 20px -8px rgba(105, 88, 164, 0.5)' : 'none'}
        borderRadius="xl"
        left={0}
        right={0}
        px={6}
      >
        <Collapse in={isOpen} animateOpacity>
          <Box py={6}>
            {toItem ? (
              <AsyncSelect
                placeholder="Search"
                chakraStyles={customStyles}
                isClearable={false}
                blurInputOnSelect={true}
                controlShouldRenderValue={false}
                menuIsOpen={true}
                loadingMessage={() => <SkeletonOptions />}
                defaultOptions={data}
                value={toItem}
                onChange={(selectedOption) => {
                  let value = {};
                  value = { ...selectedOption };
                  setToItem(value as dataType);
                  onClose();
                }}
                loadOptions={(inputValue, callback) => {
                  setTimeout(() => {
                    const values = data.filter((option) =>
                      option.label
                        .toLowerCase()
                        .includes(inputValue.toLowerCase())
                    );
                    callback(values);
                  }, 1000);
                }}
                components={{
                  DropdownIndicator,
                  IndicatorSeparator,
                  Control: CustomControl,
                  Option: CustomOption
                }}
              />
            ) : (
              <SkeletonOptions />
            )}
          </Box>
        </Collapse>
      </Box>
    </Box>
  );
};

const Rate = ({
  fromItem,
  toItem,
  tokenInputValue
}: {
  fromItem: dataType | undefined;
  toItem: dataType | undefined;
  tokenInputValue: number;
}) => {
  return (
    <Box
      bg={useColorModeValue('gray.50', 'whiteAlpha.200')}
      borderRadius="xl"
      boxShadow={useColorModeValue('0 0 2px gray', '0 0 2px white')}
      p={6}
    >
      <Flex
        justify="space-between"
        align="start"
        fontWeight="bold"
        fontSize={{ md: 'lg' }}
        color={useColorModeValue('blackAlpha.700', 'whiteAlpha.700')}
        mb={1}
      >
        <Text flex={1} mr={2}>
          Rate
        </Text>
        {fromItem && toItem ? (
          <Stack
            as="span"
            isInline
            wrap="wrap"
            maxW={{ base: 56, sm: 'initial' }}
            justify="end"
          >
            <Text>
              {tokenInputValue}&ensp;{fromItem.label}
            </Text>
            <Text>=</Text>
            <Text>3.265358&ensp;{toItem.label}</Text>
          </Stack>
        ) : (
          <Skeleton w={{ base: 32, sm: 48 }} h={{ base: 6, sm: 8 }} />
        )}
      </Flex>
      <Flex justify="end" mb={4}>
        {fromItem && toItem ? (
          <Stack
            as="span"
            isInline
            wrap="wrap"
            fontSize={{ base: 'sm', md: 'md' }}
            fontWeight="bold"
            // eslint-disable-next-line react-hooks/rules-of-hooks
            color={useColorModeValue('blackAlpha.600', 'whiteAlpha.600')}
            maxW={{ base: 56, sm: 'initial' }}
            justify="end"
          >
            <Text>3.265358&ensp;{toItem.label}</Text>
            <Text>=</Text>
            <Text>
              {tokenInputValue}&ensp;{fromItem.label}
            </Text>
          </Stack>
        ) : (
          <Skeleton w={{ base: 28, sm: 40 }} h={{ base: 4, sm: 6 }} />
        )}
      </Flex>
      <Flex
        justify="space-between"
        fontWeight="bold"
        fontSize={{ md: 'lg' }}
        color={useColorModeValue('blackAlpha.700', 'whiteAlpha.700')}
      >
        <Text>Swap Fee</Text>
        <Text>0.3%</Text>
      </Flex>
      <Divider
        borderColor={useColorModeValue('blackAlpha.400', 'whiteAlpha.600')}
        my={{ base: 4, md: 6 }}
      />
      <Flex
        justify="space-between"
        fontWeight="bold"
        fontSize={{ md: 'lg' }}
        color={useColorModeValue('blackAlpha.800', 'whiteAlpha.900')}
      >
        <Text>Estimated Slippage</Text>
        <Text>&lt;&nbsp;0.001%</Text>
      </Flex>
    </Box>
  );
};

export const CreateOption = () => {
  const [data, setData] = useState<dataType[]>([{
    label: "NIBI",
    value: "NIBI",
    denom: "unibi",
    imgSrc: "",
  },{
    label: "NUSD",
    value: "NUSD",
    denom: "unusd",
    imgSrc: "",
  }]);
  const [fromItem, setFromItem] = useState<dataType>({
    label: "NIBI",
    value: "NIBI",
    denom: "unibi",
    imgSrc: "",
  });
  const [toItem, setToItem] = useState<dataType>({
    label: "NUSD",
    value: "NUSD",
    denom: "unusd",
    imgSrc: "",
  });
  const [loading, setLoading] = useState(true);
  const [tokenInputValue, setTokenInputValue] = useState(0);

  setTimeout(() => {
    setLoading(false);
  }, 2000);
  const {assets} = useChain(chainName);
  useEffect(() => {
    const assetList = assets?.assets
      .map((asset) => {
          return {
            label: asset.name,
            value: asset.name,
            denom: asset.denom_units[0].denom,
            imgSrc: asset.logo_URIs?.svg,
            ibc: asset.ibc
          };
      })
      if(!loading){
        setData(assetList);
        setFromItem(assetList![0]);
        setToItem(assetList![1]);  
      }
      console.log(fromItem)
  }, [loading]);

  return (
    <Stack spacing={6} w="full" p={{ base: 4, sm: 6 }}>
      <Box zIndex={3000} alignSelf="end">
        <Setting />
      </Box>
      <FromToken
        data={data}
        fromItem={fromItem}
        toItem={toItem}
        tokenInputValue={tokenInputValue}
        setFromItem={setFromItem}
        setToItem={setToItem}
        setTokenInputValue={setTokenInputValue}
      />
      <ToToken data={data} toItem={toItem} setToItem={setToItem} />
      <Rate
        fromItem={fromItem}
        toItem={toItem}
        tokenInputValue={tokenInputValue}
      />
      <Button h={{ base: 12, md: 16 }} w="full" colorScheme="primary">
        create option
      </Button>
    </Stack>
  );
};

