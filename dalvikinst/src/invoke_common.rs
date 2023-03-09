pub fn decode_parameters(data: Vec<u8>) -> Vec<u32> {
    let mut parameters: Vec<u32> = vec![];

    let argument_word_count = (data[0] & 0xF0) >> 4;
    let method_idx = (data[2] as u16) << 8 | (data[1] as u16);
    let fedc = (data[4] as u16) << 8 | (data[3] as u16);

    parameters.push(argument_word_count as u32);
    parameters.push(method_idx as u32);

    for increment in 0..argument_word_count {
        if increment == 4 {
            let g = (data[0] & 0x0F) as u32;
            parameters.push(g as u32);
            break;
        }
        let value = (fedc & (0x000F << 4 * increment)) >> 4 * increment;
        parameters.push(value as u32);
    }

    parameters
}
