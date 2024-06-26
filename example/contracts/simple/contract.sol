contract MyContract {
    uint112 public plainUint112;

    uint256[] public dynamicArray;
    uint256[][] public dynamicArrayNested;

    uint32 public plainUint32;
    address public plainAddress;

    struct MyStruct {
        address myAddress;
        uint256 myUint;
    }
    struct MyStructNested {
        address myAddress;
        MyStruct myStruct;
    }
    struct MyStructSmall {
        uint32 smallInt1;
        uint32 smallInt2;
    }
    MyStructNested public myStructNested;

    uint112[10] public staticArray;
    MyStruct[2] public staticArrayLarge;
    uint8[2][4] public staticArrayNestedSmall;
    MyStructNested[] public dynamicArrayStruct;
    MyStructSmall[] public dynamicArraySmall;

    mapping(uint256 => uint256) public myMapping1;
    mapping(string => uint256) public myMapping2;
    mapping(bytes => uint64) public myMapping3;
    mapping(bool => bool) public myMappingBool;
    mapping(address => mapping(address => address)) public myAddressMappingNested;
    mapping(uint256 => mapping(uint256 => uint256)) public myNestedMapping;
//    mapping(uint256 => mapping(uint256 => MyStruct)) public myNestedMapping;
    enum MyEnum {Enum1, Enum2}
    MyEnum myEnum;
//    uint8[31] public ___gap___;
    uint256[38] public ___gap___;
    string public plainString;
    bytes public plainBytes;
    bytes32 public plainBytesN32;
    bytes1 public plainBytesN1;
}
