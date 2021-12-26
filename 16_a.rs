use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;


fn main() -> io::Result<()> {

  let mut vsum = 0;

  let in_a = "D2FE28"; // literal 2021
  let in_b = "38006F45291200"; // Operator contains 2 literals, 10 & 20
  let in_c = "EE00D40C823060"; // operator packet length type ID 1 that contains three sub-packets
  let in_d = "8A004A801A8002F478";
  let in_e = "620080001611562C8802118E34";
  let in_f = "C0015000016115A2E0802F182340";
  let in_g = "A0016C880162017C3686B18A3D4780";
  let in_actual = "E20D7880532D4E551A5791BD7B8C964C1548CB3EC1FCA41CC00C6D50024400C202A65C00C20257C008AF70024C00810039C00C3002D400A300258040F200D6040093002CC0084003FA52DB8134DE620EC01DECC4C8A5B55E204B6610189F87BDD3B30052C01493E2DC9F1724B3C1F8DC801E249E8D66C564715589BCCF08B23CA1A00039D35FD6AC5727801500260B8801F253D467BFF99C40182004223B4458D2600E42C82D07CC01D83F0521C180273D5C8EE802B29F7C9DA1DCACD1D802469FF57558D6A65372113005E4DB25CF8C0209B329D0D996C92605009A637D299AEF06622CE4F1D7560141A52BC6D91C73CD732153BF862F39BA49E6BA8C438C010E009AA6B75EF7EE53BBAC244933A48600B025AD7C074FEB901599A49808008398142013426BD06FA00D540010C87F0CA29880370E21D42294A6E3BCF0A080324A006824E3FCBE4A782E7F356A5006A587A56D3699CF2F4FD6DF60862600BF802F25B4E96BDD26049802333EB7DDB401795FC36BD26A860094E176006A0200FC4B8790B4001098A50A61748D2DEDDF4C6200F4B6FE1F1665BED44015ACC055802B23BD87C8EF61E600B4D6BAD5800AA4E5C8672E4E401D0CC89F802D298F6A317894C7B518BE4772013C2803710004261EC318B800084C7288509E56FD6430052482340128FB37286F9194EE3D31FA43BACAF2802B12A7B83E4017E4E755E801A2942A9FCE757093005A6D1F803561007A17C3B8EE0008442085D1E8C0109E3BC00CDE4BFED737A90DC97FDAE6F521B97B4619BE17CC01D94489E1C9623000F924A7C8C77EA61E6679F7398159DE7D84C015A0040670765D5A52D060200C92801CA8A531194E98DA3CCF8C8C017C00416703665A2141008CF34EF8019A080390962841C1007217C5587E60164F81C9A5CE0E4AA549223002E32BDCEA36B2E100A160008747D8B705C001098DB13A388803F1AE304600";

  let input = in_actual.clone();
  let mut bin_input = String::new();

  for c in input.chars() {
    if c == '0' { bin_input += "0000" };
    if c == '1' { bin_input += "0001" };
    if c == '2' { bin_input += "0010" };
    if c == '3' { bin_input += "0011" };
    if c == '4' { bin_input += "0100" };
    if c == '5' { bin_input += "0101" };
    if c == '6' { bin_input += "0110" };
    if c == '7' { bin_input += "0111" };
    if c == '8' { bin_input += "1000" };
    if c == '9' { bin_input += "1001" };
    if c == 'A' { bin_input += "1010" };
    if c == 'B' { bin_input += "1011" };
    if c == 'C' { bin_input += "1100" };
    if c == 'D' { bin_input += "1101" };
    if c == 'E' { bin_input += "1110" };
    if c == 'F' { bin_input += "1111" };
  }
  //println!("{}", input);
  //println!("{}", bin_input);


  let mut idx = 0;

  while (bin_input.len() - idx) > 11 {
    let (new_vsum, new_idx) = process(vsum, &bin_input, idx);
    vsum = new_vsum;
    idx = new_idx;
    //println!("{} / {}",  idx, bin_input.len());
  }
  println!("PART A: {}", vsum);
  Ok(())
}

  
fn process(mut vsum: i32, bin_input: &String, mut idx: usize) -> (i32, usize) {
  let mut start_idx = idx;
  let mut version = &bin_input[idx..idx+3];
  idx += 3;
  let mut version_dec = isize::from_str_radix(&version, 2).unwrap();
  vsum += version_dec as i32;

  //println!("version: {}", version_dec);

  let mut typeid = &bin_input[idx..idx+3];
  idx += 3;
  //println!("type: {}", typeid);

  // LITERAL
  if typeid == "100" {
    // CONSUME IN 5-BYTE CHUNKS
    //println!("LITERAL!");
    let mut not_last_chunk = 1;
    let mut literal = String::new();
    while not_last_chunk == 1 {
      not_last_chunk = bin_input[idx..idx+1].parse::<i32>().unwrap();
      literal += &bin_input[idx+1..idx+5].to_string();
      idx += 5;
    }
    let literal_dec = isize::from_str_radix(&literal, 2).unwrap();
    //println!("literal: {}", literal_dec);
    //println!("   idx : {}", idx);

    //let mut leftovers = bin_input.len() - idx;
    //let length = (idx - start_idx) as i32;
    //let mut bytes = (length / 4) as i32;
    //bytes += 1;
    //let total_length = bytes * 4;
    ////println!("length: {}", length);
    //println!("total_length: {}", total_length);
    //let leftovers = total_length - length;
    //println!("leftovers: {}", leftovers);
    //idx += leftovers as usize;
  } else {
    // OPERATOR
    //println!("OPERATOR!");
    let mut length_type = bin_input[idx..idx+1].parse::<i32>().unwrap();
    idx += 1;
    if length_type == 1 {
      // PROCESS X PACKETS
      let num_packets_bin = &bin_input[idx..idx+11].to_string();
      idx += 11;
      let num_packets = isize::from_str_radix(&num_packets_bin, 2).unwrap();
      //println!("PROCESS {} PACKETS", num_packets);
      for i in 0..num_packets {
        //println!("PROCESSING SUB-PACKET...");
        let (new_vsum, new_idx) = process(vsum, &bin_input, idx);
        vsum = new_vsum;
        idx = new_idx;
      }
    } else {
      // PROCESS X BITS
      let num_bits_bin = &bin_input[idx..idx+15].to_string();
      idx += 15;
      let num_bits = isize::from_str_radix(&num_bits_bin, 2).unwrap();
      //println!("PROCESS {} BITS", num_bits);
      let start_idx = idx;
      while idx as i32 != (start_idx as i32) + (num_bits as i32) {
        //println!("PROCESSING SUB-PACKET...");
        //println!("  idx = {} ; start_idx = {} ; num_bits = {}", idx, start_idx, num_bits);
        let (new_vsum, new_idx) = process(vsum, &bin_input, idx);
        vsum = new_vsum;
        idx = new_idx;
        //println!(" end idx = {}", idx);
      }
    }
  }

  return (vsum, idx);
  
}


