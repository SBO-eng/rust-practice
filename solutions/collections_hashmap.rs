use std::collections::HashMap;

fn main() {
    // 1) Basic ops: fix types + options
    {
        let mut scores = HashMap::new();
        scores.insert("Sunface", 98);
        scores.insert("Daniel", 95);
        scores.insert("Ashley", 69);
        scores.insert("Katie", 58);

        let score = scores.get("Sunface");
        assert_eq!(score, Some(&98));

        if scores.contains_key("Daniel") {
            let score = scores["Daniel"];
            assert_eq!(score, 95);
            scores.remove("Daniel");
        }

        assert_eq!(scores.len(), 3);

        for (name, score) in scores {
            println!("The score of {} is {}", name, score);
        }
    }

    // 2) Build map2 using collect
    {
        let teams = [
            ("Chinese Team", 100),
            ("American Team", 10),
            ("France Team", 50),
        ];

        let mut teams_map1 = HashMap::new();
        for team in &teams {
            teams_map1.insert(team.0, team.1);
        }

        let teams_map2: HashMap<_, _> = teams.into_iter().collect();

        assert_eq!(teams_map1, teams_map2);
        println!("Success!");
    }

    // 3) entry/or_insert
    {
        let mut player_stats = HashMap::new();

        player_stats.entry("health").or_insert(100);
        assert_eq!(player_stats["health"], 100);

        player_stats.entry("health").or_insert_with(random_stat_buff);
        assert_eq!(player_stats["health"], 100);

        let health = player_stats.entry("health").or_insert(50);
        assert_eq!(*health, 100);
        *health -= 50;
        assert_eq!(*health, 50);

        println!("Success!");
    }

    // 4) custom key needs Eq + Hash + Debug
    {
        #[derive(Debug, Hash, PartialEq, Eq)]
        struct Viking {
            name: String,
            country: String,
        }

        impl Viking {
            fn new(name: &str, country: &str) -> Viking {
                Viking {
                    name: name.to_string(),
                    country: country.to_string(),
                }
            }
        }

        let vikings = HashMap::from([
            (Viking::new("Einar", "Norway"), 25),
            (Viking::new("Olaf", "Denmark"), 24),
            (Viking::new("Harald", "Iceland"), 12),
        ]);

        for (viking, health) in &vikings {
            println!("{:?} has {} hp", viking, health);
        }
    }

    // 5) ownership moved: fix with least changes
    {
        let v1 = 10;
        let mut m1 = HashMap::new();
        m1.insert(v1, v1);
        println!("v1 is still usable after inserting to hashmap : {}", v1);

        let v2 = "hello".to_string();
        let mut m2 = HashMap::new();
        m2.insert(v2.clone(), v1);

        assert_eq!(v2, "hello");
        println!("Success!");
    }

    println!("collections_hashmap OK ✅");
}

fn random_stat_buff() -> u8 {
    42
}
