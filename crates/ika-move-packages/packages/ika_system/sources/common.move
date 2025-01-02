module ika_system::common;

public(package) fun bcs_output_u32_as_uleb128(bcs_bytes: &mut vector<u8>, value: u32) {
    let mut value = value;
    while (value >= 0x80) {
        // Write 7 (lowest) bits of data and set the 8th bit to 1.
        let byte = (value & 0x7f) as u8;
        bcs_bytes.append(vector[byte | 0x80]);
        value = value >> 7;
    };
    // Write the remaining bits of data and set the highest bit to 0.
    bcs_bytes.append(vector[value as u8]);
}