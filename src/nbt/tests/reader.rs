use crate::nbt::reader::NbtReader;




#[test]
fn test_string(){
    
    dbg!(NbtReader::read_bytes(&[0x0a, 0x00, 0x05, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x07, 0x00, 0x06, 0x64, 0x65, 0x6e, 0x65, 0x6d, 
        0x65, 0x00, 0x00, 0x00, 0x0c, 0x12, 0x20, 0x30, 0x40, 0x50, 0x65, 0x80, 0x00, 0x51, 0x05, 0x41, 
        0x65, 0x02, 0x00, 0x05, 0x73, 0x68, 0x6f, 0x72, 0x74, 0x00, 0x1f, 0x06, 0x00, 0x06, 0x33, 0x31, 
        0x2e, 0x33, 0x31, 0x31, 0x40, 0x51, 0x6c, 0xbc, 0x6a, 0x7e, 0xf9, 0xdb, 0x09, 0x00, 0x04, 0x6c, 
        0x69, 0x73, 0x74, 0x08, 0x00, 0x00, 0x00, 0x03, 0x00, 0x02, 0x73, 0x31, 0x00, 0x02, 0x73, 0x32, 
        0x00, 0x02, 0x73, 0x33, 0x00]).unwrap());
}