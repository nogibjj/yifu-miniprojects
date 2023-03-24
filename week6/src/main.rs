use std::io;

fn main() {
    println!("Welcome to the text-based game!");

    let mut player_health = 10;
    let player_damage = 2;
    let mut enemy_health = 5;
    let enemy_damage = 1;

    loop {
        println!("You are currently in a room. What would you like to do?");

        println!("1. Attack the enemy");
        println!("2. Rest and recover health");
        println!("3. Quit the game");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number");
                continue;
            }
        };

        match choice {
            1 => {
                let player_hit = rand::random::<u32>() % 2 == 0;
                let enemy_hit = rand::random::<u32>() % 2 == 0;

                if player_hit {
                    enemy_health -= player_damage;
                    println!("You hit the enemy for {} damage!", player_damage);
                } else {
                    println!("You missed!");
                }

                if enemy_hit {
                    player_health -= enemy_damage;
                    println!("The enemy hit you for {} damage!", enemy_damage);
                } else {
                    println!("The enemy missed!");
                }

                if player_health <= 0 {
                    println!("Game over! You have been defeated");
                    break;
                }

                if enemy_health <= 0 {
                    println!("Congratulations! You have defeated the enemy");
                    break;
                }
            }
            2 => {
                player_health += 2;
                println!("You recover 2 health points");
            }
            3 => {
                println!("Thanks for playing!");
                break;
            }
            _ => {
                println!("Invalid input! Please choose a valid option");
                continue;
            }
        }
    }
}

#[test]
fn test_player_attack() {
    let _player_health = 10;
    let player_damage = 2;
    let mut enemy_health = 5;

    // Simulate the player attacking the enemy
    enemy_health -= player_damage;

    // Verify that the enemy's health has decreased
    assert_eq!(enemy_health, 3);
}

#[test]
fn test_enemy_attack() {
    let mut player_health = 10;
    let enemy_damage = 1;

    // Simulate the enemy attacking the player
    player_health -= enemy_damage;

    // Verify that the player's health has decreased
    assert_eq!(player_health, 9);
}
