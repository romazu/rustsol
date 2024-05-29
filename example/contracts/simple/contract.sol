contract MyContract {
    uint112 public plainUint112;
    uint32 public plainUint32;
    string public plainString;

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
    uint256[] public dynamicArray;
    MyStructNested[] public dynamicArrayStruct;
    MyStructSmall[] public dynamicArraySmall;

    mapping(uint256 => uint256) public myMapping1;
    mapping(string => uint256) public myMapping2;
    mapping(uint256 => mapping(uint256 => uint256)) public myNestedMapping;
//    mapping(uint256 => mapping(uint256 => MyStruct)) public myNestedMapping;
}
