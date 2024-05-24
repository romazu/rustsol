contract MyContract {
    uint112 public plainUint112;
    uint32 public plainUint32;
    string public plainString;

    struct MyStruct {
        address myAddress;
        uint256 myUint;
    }
    MyStruct public myStruct;

    uint112[10] public staticArray;
    uint256[] public dynamicArray;

    mapping(string => uint256) public myMapping;
    mapping(uint256 => mapping(uint256 => MyStruct)) public myNestedMapping;
}
