use std::fs;

struct Rules {
    departure_location: [(u32, u32); 2],
    departure_station: [(u32, u32); 2],
    departure_platform: [(u32, u32); 2],
    departure_track: [(u32, u32); 2],
    departure_date: [(u32, u32); 2],
    departure_time: [(u32, u32); 2],
    arrival_location: [(u32, u32); 2],
    arrival_station: [(u32, u32); 2],
    arrival_platform: [(u32, u32); 2],
    arrival_track: [(u32, u32); 2],
    class: [(u32, u32); 2],
    duration: [(u32, u32); 2],
    price: [(u32, u32); 2],
    route: [(u32, u32); 2],
    row: [(u32, u32); 2],
    seat: [(u32, u32); 2],
    train: [(u32, u32); 2],
    ticket_type: [(u32, u32); 2],
    wagon: [(u32, u32); 2],
    zone: [(u32, u32); 2],
}

fn is_in_range(range: [(u32, u32); 2], number: u32) -> bool {
    (number >= range[0].0 && number <= range[0].1) || (number >= range[1].0 && number <= range[1].1)
}

impl Rules {
    fn is_correct_departure_location(&self, number: u32) -> bool {
        is_in_range(self.departure_location, number)
    }

    fn is_correct_departure_station(&self, number: u32) -> bool {
        is_in_range(self.departure_station, number)
    }

    fn is_correct_departure_platform(&self, number: u32) -> bool {
        is_in_range(self.departure_platform, number)
    }

    fn is_correct_departure_track(&self, number: u32) -> bool {
        is_in_range(self.departure_track, number)
    }

    fn is_correct_departure_date(&self, number: u32) -> bool {
        is_in_range(self.departure_date, number)
    }

    fn is_correct_departure_time(&self, number: u32) -> bool {
        is_in_range(self.departure_time, number)
    }

    fn is_correct_arrival_location(&self, number: u32) -> bool {
        is_in_range(self.arrival_location, number)
    }

    fn is_correct_arrival_station(&self, number: u32) -> bool {
        is_in_range(self.arrival_station, number)
    }

    fn is_correct_arrival_platform(&self, number: u32) -> bool {
        is_in_range(self.arrival_platform, number)
    }

    fn is_correct_arrival_track(&self, number: u32) -> bool {
        is_in_range(self.arrival_track, number)
    }

    fn is_correct_class(&self, number: u32) -> bool {
        is_in_range(self.class, number)
    }

    fn is_correct_duration(&self, number: u32) -> bool {
        is_in_range(self.duration, number)
    }

    fn is_correct_price(&self, number: u32) -> bool {
        is_in_range(self.price, number)
    }

    fn is_correct_route(&self, number: u32) -> bool {
        is_in_range(self.route, number)
    }

    fn is_correct_row(&self, number: u32) -> bool {
        is_in_range(self.row, number)
    }

    fn is_correct_seat(&self, number: u32) -> bool {
        is_in_range(self.seat, number)
    }

    fn is_correct_train(&self, number: u32) -> bool {
        is_in_range(self.train, number)
    }

    fn is_correct_ticket_type(&self, number: u32) -> bool {
        is_in_range(self.ticket_type, number)
    }

    fn is_correct_wagon(&self, number: u32) -> bool {
        is_in_range(self.wagon, number)
    }

    fn is_correct_zone(&self, number: u32) -> bool {
        is_in_range(self.zone, number)
    }
}

fn part_1(tickets: &str, rules: &Rules) -> u32 {
    let is_correct = |n| {
        rules.is_correct_departure_location(n)
            || rules.is_correct_departure_station(n)
            || rules.is_correct_departure_platform(n)
            || rules.is_correct_departure_track(n)
            || rules.is_correct_departure_date(n)
            || rules.is_correct_departure_time(n)
            || rules.is_correct_arrival_location(n)
            || rules.is_correct_arrival_station(n)
            || rules.is_correct_arrival_platform(n)
            || rules.is_correct_arrival_track(n)
            || rules.is_correct_class(n)
            || rules.is_correct_duration(n)
            || rules.is_correct_price(n)
            || rules.is_correct_route(n)
            || rules.is_correct_row(n)
            || rules.is_correct_seat(n)
            || rules.is_correct_train(n)
            || rules.is_correct_ticket_type(n)
            || rules.is_correct_wagon(n)
            || rules.is_correct_zone(n)
    };

    tickets
        .lines()
        .flat_map(|l| l.split(','))
        .map(|n| n.parse::<u32>().unwrap())
        .filter(|&n| !is_correct(n))
        .sum::<u32>()
}

fn part_2(tickets: &str, rules: &Rules) {
    let is_correct = |n| {
        rules.is_correct_departure_location(n)
            || rules.is_correct_departure_station(n)
            || rules.is_correct_departure_platform(n)
            || rules.is_correct_departure_track(n)
            || rules.is_correct_departure_date(n)
            || rules.is_correct_departure_time(n)
            || rules.is_correct_arrival_location(n)
            || rules.is_correct_arrival_station(n)
            || rules.is_correct_arrival_platform(n)
            || rules.is_correct_arrival_track(n)
            || rules.is_correct_class(n)
            || rules.is_correct_duration(n)
            || rules.is_correct_price(n)
            || rules.is_correct_route(n)
            || rules.is_correct_row(n)
            || rules.is_correct_seat(n)
            || rules.is_correct_train(n)
            || rules.is_correct_ticket_type(n)
            || rules.is_correct_wagon(n)
            || rules.is_correct_zone(n)
    };

    let tickets = tickets
        .lines()
        .map(|ticket| {
            ticket
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let valid_tickets = tickets
        .iter()
        .filter(|&ticket| ticket.iter().all(|n| is_correct(*n)))
        .collect::<Vec<&Vec<u32>>>();

    let fn_table = [
        is_correct_r1,
        is_correct_r2,
        is_correct_r3,
        is_correct_r4,
        is_correct_r5,
        is_correct_r6,
        is_correct_r7,
        is_correct_r8,
        is_correct_r9,
        is_correct_r10,
        is_correct_r11,
        is_correct_r12,
        is_correct_r13,
        is_correct_r14,
        is_correct_r15,
        is_correct_r16,
        is_correct_r17,
        is_correct_r18,
        is_correct_r19,
        is_correct_r20,
    ];

    let used = vec![];
    for i in 0..20 {
        verify(&fn_table, &valid_tickets, used.clone(), i);
    }
}

fn is_correct_r1(number: u32) -> bool {
    // departure location: 36-363 or 377-962
    is_in_range([(36, 363), (377, 962)], number)
}
fn is_correct_r2(number: u32) -> bool {
    // departure station: 29-221 or 234-953
    is_in_range([(29, 221), (234, 953)], number)
}
fn is_correct_r3(number: u32) -> bool {
    // departure platform: 39-585 or 595-954
    is_in_range([(39, 585), (595, 954)], number)
}
fn is_correct_r4(number: u32) -> bool {
    // departure track: 31-727 or 753-952
    is_in_range([(31, 727), (753, 952)], number)
}
fn is_correct_r5(number: u32) -> bool {
    // departure date: 33-862 or 883-964
    is_in_range([(33, 862), (883, 964)], number)
}
fn is_correct_r6(number: u32) -> bool {
    // departure time: 35-716 or 722-971
    is_in_range([(35, 716), (722, 971)], number)
}
fn is_correct_r7(number: u32) -> bool {
    // arrival location: 32-59 or 74-955
    is_in_range([(32, 59), (74, 955)], number)
}
fn is_correct_r8(number: u32) -> bool {
    // arrival station: 41-330 or 353-963
    is_in_range([(41, 330), (353, 963)], number)
}
fn is_correct_r9(number: u32) -> bool {
    // arrival platform: 28-883 or 894-964
    is_in_range([(28, 883), (894, 964)], number)
}
fn is_correct_r10(number: u32) -> bool {
    // arrival track: 26-669 or 691-974
    is_in_range([(26, 669), (691, 974)], number)
}
fn is_correct_r11(number: u32) -> bool {
    // class: 43-250 or 261-966
    is_in_range([(43, 250), (261, 966)], number)
}
fn is_correct_r12(number: u32) -> bool {
    // duration: 48-521 or 533-974
    is_in_range([(48, 521), (533, 974)], number)
}
fn is_correct_r13(number: u32) -> bool {
    // price: 48-100 or 107-971
    is_in_range([(48, 100), (107, 971)], number)
}
fn is_correct_r14(number: u32) -> bool {
    // route: 47-757 or 777-971
    is_in_range([(47, 757), (777, 971)], number)
}
fn is_correct_r15(number: u32) -> bool {
    // row: 38-629 or 637-961
    is_in_range([(38, 629), (637, 961)], number)
}
fn is_correct_r16(number: u32) -> bool {
    // seat: 43-310 or 330-949
    is_in_range([(43, 310), (330, 949)], number)
}
fn is_correct_r17(number: u32) -> bool {
    // train: 27-560 or 566-957
    is_in_range([(27, 560), (566, 957)], number)
}
fn is_correct_r18(number: u32) -> bool {
    // type: 50-433 or 457-963
    is_in_range([(50, 433), (457, 963)], number)
}
fn is_correct_r19(number: u32) -> bool {
    // wagon: 35-898 or 907-957
    is_in_range([(35, 898), (907, 957)], number)
}
fn is_correct_r20(number: u32) -> bool {
    // zone: 48-354 or 362-961
    is_in_range([(48, 354), (362, 961)], number)
}

fn verify(
    table: &[fn(u32) -> bool],
    tickets: &Vec<&Vec<u32>>,
    mut used: Vec<usize>,
    current: usize,
) {
    if !tickets
        .iter()
        .all(|ticket| (table[current])(*ticket.iter().nth(used.len()).unwrap()))
    {
        return;
    }
    used.push(current);

    if used.len() == 20 {
        let p0 = used.iter().position(|&p| p == 0).unwrap();
        let p1 = used.iter().position(|&p| p == 1).unwrap();
        let p2 = used.iter().position(|&p| p == 2).unwrap();
        let p3 = used.iter().position(|&p| p == 3).unwrap();
        let p4 = used.iter().position(|&p| p == 4).unwrap();
        let p5 = used.iter().position(|&p| p == 5).unwrap();
        let my_ticket: [usize; 20] = [
            89, 179, 173, 167, 157, 127, 163, 113, 137, 109, 151, 131, 97, 149, 107, 83, 79, 139,
            59, 53,
        ];
        let result = my_ticket[p0]
            * my_ticket[p1]
            * my_ticket[p2]
            * my_ticket[p3]
            * my_ticket[p4]
            * my_ticket[p5];
        assert_eq!(result, 3_765_150_732_757);
        return;
    }

    for i in 0..20 {
        if used.contains(&i) {
            continue;
        }

        verify(table, tickets, used.clone(), i);
    }
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let rules = Rules {
        departure_location: [(36, 363), (377, 962)],
        departure_station: [(29, 221), (234, 953)],
        departure_platform: [(39, 585), (595, 954)],
        departure_track: [(31, 727), (753, 952)],
        departure_date: [(33, 862), (883, 964)],
        departure_time: [(35, 716), (722, 971)],
        arrival_location: [(32, 59), (74, 955)],
        arrival_station: [(41, 330), (353, 963)],
        arrival_platform: [(28, 883), (894, 964)],
        arrival_track: [(26, 669), (691, 974)],
        class: [(43, 250), (261, 966)],
        duration: [(48, 521), (533, 974)],
        price: [(48, 100), (107, 971)],
        route: [(47, 757), (777, 971)],
        row: [(38, 629), (637, 961)],
        seat: [(43, 310), (330, 949)],
        train: [(27, 560), (566, 957)],
        ticket_type: [(50, 433), (457, 963)],
        wagon: [(35, 898), (907, 957)],
        zone: [(48, 354), (362, 961)],
    };

    assert_eq!(part_1(&content, &rules), 23_044);
    part_2(&content, &rules);
}
