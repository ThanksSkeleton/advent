struct Packet 
{
    version: i32,
    type_id: i32,

    literal: i64,

    //length_type_id: String,
    sub_packets: Vec<Packet>
}

impl Packet 
{
    fn pretty_print(&self) 
    {
        println!("Version: {}", self.version);
        println!("TypeId: {}", self.type_id);
        println!("Literal: {}", self.literal);

        for s in &self.sub_packets
        {
            println!("Sub Packet:");
            s.pretty_print();
        }
    }

    fn evaluate(&self) -> i64 
    {
        let v = &self.sub_packets;
        let evaluated: Vec<i64> = v.into_iter().map(|p| p.evaluate()).collect();

        match self.type_id 
        {
            0 => return evaluated.into_iter().sum(),
            1 => return evaluated.into_iter().fold(1, |acc, p| acc * p),
            2 => return evaluated.into_iter().min().unwrap(),
            3 => return evaluated.into_iter().max().unwrap(),
            4 => return self.literal,
            5 => return if evaluated.get(0).unwrap() > evaluated.get(1).unwrap() { 1 } else { 0 } ,
            6 => return if evaluated.get(0).unwrap() < evaluated.get(1).unwrap()  { 1 } else { 0 } ,
            7 => return if evaluated.get(0).unwrap() == evaluated.get(1).unwrap()  { 1 } else { 0 } ,
            _ => panic!()
        }
    }
}

pub fn day_16(lines: Vec<String>)
{
    let mut bitstream = 
    lines.get(0).unwrap().
    chars().
    map(|c| to_bits(c)).
    flatten();

    let p = build_packet_recursively(&mut bitstream);

    println!("{} eval", p.evaluate());
}

fn build_packet_recursively(bit_stream : &mut dyn Iterator<Item = char>) -> Packet
{
    let version: String = bit_stream.take(3).collect();

    let type_literal = "100";

    let type_id: String = bit_stream.take(3).collect();

    if type_id == type_literal 
    {
        let mut complete_string = String::from("");

        let mut complete_flag = 'X';
        while complete_flag != '0' 
        {   
            complete_flag = bit_stream.next().unwrap();

            let next_portion: String =bit_stream.take(4).collect();          
            complete_string.push_str(next_portion.as_str());
        }

        return packet_literal(version, type_id, complete_string);
    } 
    else 
    {
        let length_type_id = bit_stream.next().unwrap();

        let mut child_packets = Vec::new();

        if length_type_id == '0' // subpacket length by bits
        {
            let length_bin : String = bit_stream.take(15).collect();
            let length_int = usize::from_str_radix(&length_bin, 2).unwrap();

            let sub_iterator = bit_stream.take(length_int);
            let mut peekable_sub_iterator = sub_iterator.peekable().into_iter();
            while peekable_sub_iterator.peek().is_some()
            {
                child_packets.push(build_packet_recursively(&mut peekable_sub_iterator));
            }
        } 
        else 
        { // explicit number of subpackets

            let count_bin : String  = bit_stream.take(11).collect();
            let count_int = usize::from_str_radix(&count_bin, 2).unwrap();

            for _i in 0..count_int 
            {
                child_packets.push(build_packet_recursively(bit_stream));
            }
        }

        return packet_recursive(version, type_id, child_packets);
    }
}

fn get_version_sum(p: Packet) -> i32 
{
    let mut to_return = p.version;

    for sp in p.sub_packets 
    {
        to_return = to_return + get_version_sum(sp);
    }

    return to_return;
}

fn packet_literal(version: String, type_id: String, literal: String) -> Packet
{
    return Packet 
    { 
        version: i32::from_str_radix(&version, 2).unwrap(),
        type_id: i32::from_str_radix(&type_id, 2).unwrap(),
        literal: i64::from_str_radix(&literal, 2).unwrap(),
        sub_packets: Vec::new()
    };
}

fn packet_recursive(version: String, type_id: String, sub_packets : Vec<Packet>) -> Packet 
{
    return Packet 
    { 
        version: i32::from_str_radix(&version, 2).unwrap(),
        type_id: i32::from_str_radix(&type_id, 2).unwrap(),
        literal: 0,
        sub_packets: sub_packets
    };
}

fn to_bits(c: char) -> [char; 4]
{
    match c 
    {
        '0' => [ '0','0','0','0'],
        '1' => [ '0','0','0','1'],
        '2' => [ '0','0','1','0'],
        '3' => [ '0','0','1','1'],
        '4' => [ '0','1','0','0'],
        '5' => [ '0','1','0','1'],
        '6' => [ '0','1','1','0'],
        '7' => [ '0','1','1','1'],
        '8' => [ '1','0','0','0'],
        '9' => [ '1','0','0','1'],
        'A' => [ '1','0','1','0'],
        'B' => [ '1','0','1','1'],
        'C' => [ '1','1','0','0'],
        'D' => [ '1','1','0','1'],
        'E' => [ '1','1','1','0'],
        'F' => [ '1','1','1','1'],
        _ => panic!()
    }
}