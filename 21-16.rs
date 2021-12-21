use std::io::BufRead;

#[derive(Debug)]
struct Packet {
    version : u32,
    packet_type: u8,
    value: u128,
    children: Vec<Packet>
}

fn parse_packet(data: &str,mut i:usize) -> (Packet,usize) { 
    // parse header
    let v = u32::from_str_radix(&data[i..i+3], 2).unwrap();
    i+=3;
    let t = u8::from_str_radix(&data[i..i+3], 2).unwrap();
    i+=3;
    if t == 4 {
        // parse data literal
        let mut d = 0_u128;
        loop {
            d = d * 16;
	    let is_final_block = &data[i..i+1] == "0";
            let group = u8::from_str_radix(&data[i + 1..i + 5], 2).unwrap();
            d += group as u128;
	    i += 5;
            if is_final_block {
                break;
            }

        }
	let packet = Packet { version:v, packet_type:t, value:d, children:vec![] };
	return (packet,i);
    }
    else {
	// operator
	let length_type = &data[i..i+1];
	i+=1;
	if length_type=="0" {
	    let length = usize::from_str_radix(&data[i..i+15],2).unwrap();
	    i+=15;
	    let mut cs : Vec<Packet> = Vec::new();
	    let endpos = i+length;
	    while i<endpos-1 {
		let (p,pos) = parse_packet(data,i);
		cs.push(p);
		i=pos;
	    }
	    let packet = Packet {version:v, packet_type:t, value:0, children:cs};
	    return (packet,i);
	}
	else {
	    let count = usize::from_str_radix(&data[i..i+11],2).unwrap();
	    i+=11;
	    let mut cs : Vec<Packet> = Vec::new();
	    for packet_count in 0..count {
		let (p,pos) = parse_packet(data,i);
		cs.push(p);
		i=pos;			
	    }
	    let packet = Packet {version:v, packet_type:t, value:0, children:cs};
	    return (packet,i);
	};
    }
}

fn version_sums(p:&Packet) -> u32 {
    (p.version) + p.children.iter().map(|c|version_sums(c)).sum::<u32>()
}

fn eval(p:&Packet) -> u128 {
    let mut cs = p.children.iter().map(|c|eval(c));
    match p.packet_type {
	0 => cs.sum::<u128>(),
	1 => cs.product::<u128>(),
	2 => cs.min().unwrap(),
	3 => cs.max().unwrap(),
	4 => p.value,
	5 => if cs.next().unwrap() > cs.next().unwrap() {1} else {0},
	6 => if cs.next().unwrap() < cs.next().unwrap() {1} else {0},
	7 => if cs.next().unwrap() == cs.next().unwrap() {1} else {0},
	_ => panic!("oops")
    }	
}

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let data : String = "420D50000B318100415919B24E72D6509AE67F87195A3CCC518CC01197D538C3E00BC9A349A09802D258CC16FC016100660DC4283200087C6485F1C8C015A00A5A5FB19C363F2FD8CE1B1B99DE81D00C9D3002100B58002AB5400D50038008DA2020A9C00F300248065A4016B4C00810028003D9600CA4C0084007B8400A0002AA6F68440274080331D20C4300004323CC32830200D42A85D1BE4F1C1440072E4630F2CCD624206008CC5B3E3AB00580010E8710862F0803D06E10C65000946442A631EC2EC30926A600D2A583653BE2D98BFE3820975787C600A680252AC9354FFE8CD23BE1E180253548D057002429794BD4759794BD4709AEDAFF0530043003511006E24C4685A00087C428811EE7FD8BBC1805D28C73C93262526CB36AC600DCB9649334A23900AA9257963FEF17D8028200DC608A71B80010A8D50C23E9802B37AA40EA801CD96EDA25B39593BB002A33F72D9AD959802525BCD6D36CC00D580010A86D1761F080311AE32C73500224E3BCD6D0AE5600024F92F654E5F6132B49979802129DC6593401591389CA62A4840101C9064A34499E4A1B180276008CDEFA0D37BE834F6F11B13900923E008CF6611BC65BCB2CB46B3A779D4C998A848DED30F0014288010A8451062B980311C21BC7C20042A2846782A400834916CFA5B8013374F6A33973C532F071000B565F47F15A526273BB129B6D9985680680111C728FD339BDBD8F03980230A6C0119774999A09001093E34600A60052B2B1D7EF60C958EBF7B074D7AF4928CD6BA5A40208E002F935E855AE68EE56F3ED271E6B44460084AB55002572F3289B78600A6647D1E5F6871BE5E598099006512207600BCDCBCFD23CE463678100467680D27BAE920804119DBFA96E05F00431269D255DDA528D83A577285B91BCCB4802AB95A5C9B001299793FCD24C5D600BC652523D82D3FCB56EF737F045008E0FCDC7DAE40B64F7F799F3981F2490"
//    "D2FE28"
//	"38006F45291200"
    //	"EE00D40C823060"
    //	"8A004A801A8002F478"
	//"620080001611562C8802118E34"
//	"C0015000016115A2E0802F182340"
//	"A0016C880162017C3686B18A3D4780"
	.chars()
	.map(|c|u8::from_str_radix(&c.to_string(),16).unwrap())
	.map(|i|format!("{:04b}",i))
	.collect::<Vec<String>>()
	.join("");

    let (p,_) = parse_packet(&data,0);
    println!("{:#?}",p);
    println!("sol1 version sum: {}",version_sums(&p));
    println!("sol2 eval: {}",eval(&p));
}

