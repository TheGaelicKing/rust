// Modules
use console::Style;
use console::Term;
use rand::rngs::ThreadRng;
use rand::Rng;

// Panic message
const PANIC_MSG: &str = "An error has occurred";

// Main entry point
fn main() {
    // Create references
    let term = Term::stdout();
    let mut rng = rand::thread_rng();

    // Styles
    let styles = [
        Style::new().bold(),
        Style::new().red(),
        Style::new().blue(),
        Style::new().green(),
        Style::new().yellow(),
    ]; // 0: bold; 1: red; 2: blue; 3: green; 4: yellow

    // Player stats
    let mut player_stats = [100]; // 0: money

    // Begin game loop
    clear(&term);
    draw_header(&styles);
    println!("Welcome to the casino!");
    wait(&term, &styles);
    loop {
        let choice = &draw_menu_screen(&term, &styles, &mut player_stats) as &str;
        match choice {
            "1" => blackjack(&term, &styles, &rng, &mut player_stats),
            "2" => lottery(&term, &styles, &mut rng, &mut player_stats),
            "3" => {
                break;
            }
            _ => {
                println!("{}", styles[1].apply_to("\nInvalid input received"));
                break;
            }
        };
    }

    // Exit the terminal
    println!("{}", styles[1].apply_to("\nThe terminal will exit"));
    wait(&term, &styles);
}

// Blackjack
fn blackjack(term: &Term, styles: &[Style; 5], _rng: &ThreadRng, _stats: &mut [i32; 1]) {
    // Draw elements
    clear(term);
    draw_header(styles);
    println!("Welcome to blackjack!\n\nThe goal of the game is to reach cards which value the closet to 21 without going over this number.\n\nPress [h] to hit\nPress [s] to stand");
    wait(term, styles);
}

// Lottery
fn lottery(term: &Term, styles: &[Style; 5], rng: &mut ThreadRng, stats: &mut [i32; 1]) {
    // Draw elements
    clear(term);
    draw_header(styles);
    println!("Welcome to the lottery!\n\nThe goal of the game is to purchase winning tickets.\n\n[Enter] the number of tickets you would like to buy");
    wait(term, styles);

    // Prompt for tickets
    let jackpot = rng.gen_range(((stats[0] * 3) / 4)..=((stats[0] * 4) / 1));
    let ticket_price = rng.gen_range(((stats[0] * 1) / 20)..=((stats[0] * 1) / 10));
    clear(term);
    draw_header(styles);
    println!(
        "{0}{1}\n\nThe jackpot is currently {2} money\nThe ticket price is currently {3} money\n\nEnter the number of tickets to buy:\n",
        styles[4].apply_to("Money: "),
        styles[4].apply_to(stats[0]),
        styles[0].apply_to(jackpot),
        styles[0].apply_to(ticket_price)
    );
    let num_tickets: i32 = term.read_line().expect(PANIC_MSG).parse().expect(PANIC_MSG);

    // Skip rolling if no tickets have been purchased
    if num_tickets <= 0 {
        println!(
            "{}",
            styles[1].apply_to("\nInvalid number of tickets entered")
        );
        wait(term, styles);
        return;
    }

    // Present total price
    let total_price = ticket_price * num_tickets;
    if total_price > stats[0] {
        println!(
            "{}",
            styles[1].apply_to("\nYou can't afford this many tickets")
        );
        wait(term, styles);
        return;
    }
    stats[0] -= total_price;
    println!(
        "\nYou have bought {0} tickets for {1} money",
        styles[0].apply_to(num_tickets),
        styles[0].apply_to(total_price)
    );
    wait(term, styles);

    // Roll tickets
    let mut won_jackpot = false;
    for _i in 0..num_tickets {
        let jackpot_seed = rng.gen_range(0..=100);
        if jackpot_seed > 95 {
            won_jackpot = true;
            break;
        }
    }
    if won_jackpot {
        stats[0] += jackpot;
        println!("{}", styles[3].apply_to("\nYou won the jackpot!"));
    } else {
        println!("{}", styles[1].apply_to("\nYou won nothing"));
    }
    wait(term, styles);
}

// Title
fn draw_menu_screen(term: &Term, styles: &[Style; 5], stats: &mut [i32; 1]) -> String {
    // Draw elements
    clear(term);
    draw_header(styles);
    println!(
        "{0}{1}\n\nChoose an option:\n\n[1] Blackjack\n[2] Lottery\n[3] Exit\n",
        styles[4].apply_to("Money: "),
        styles[4].apply_to(stats[0])
    );

    // Prompt for user choice
    let choice = term.read_line().expect(PANIC_MSG);
    choice
}

// Header
fn draw_header(styles: &[Style; 5]) {
    // Title
    println!(
        "{}",
        styles[0].apply_to("========== Casino v0.0.1 ==========\n")
    );
}

// Clear terminal
fn clear(term: &Term) {
    term.clear_screen().expect(PANIC_MSG);
}

// Wait for input
fn wait(term: &Term, styles: &[Style; 5]) {
    println!("{}", styles[0].apply_to("\nPress any key to continue"));
    term.read_char().expect(PANIC_MSG);
}
