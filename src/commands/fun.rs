use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::Rng;

/// fun stuff i guess
#[poise::command(
    slash_command,
    subcommands(
        "coinflip", "dice", "eightball", "joke", "fact",
        "cat", "dog", "fox", "panda", "bird",
        "cookie", "hug", "pat", "kiss", "slap",
        "meme", "ship", "rate", "mock", "reverse", "ascii",
        "choose", "trivia", "roast", "compliment"
    ),
    category = "fun"
)]
pub async fn fun(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// flip a coin lol
#[poise::command(slash_command)]
pub async fn coinflip(ctx: Context<'_>) -> Result<(), Error> {
    let (result, emoji) = if rand::random::<bool>() {
        ("Heads", "🪙")
    } else {
        ("Tails", "🥈")
    };
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("{} Coin Flip", emoji))
            .description(format!("The coin landed on **{}**!", result))
            .footer(serenity::CreateEmbedFooter::new("50/50 odds, no house edge"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// roll some dice idk
#[poise::command(slash_command)]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "Number of sides (defaults to 6)"] sides: Option<u32>,
) -> Result<(), Error> {
    let sides = sides.unwrap_or(6).max(2);
    let result = rand::thread_rng().gen_range(1..=sides);
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🎲 Dice Roll")
            .description(format!("You rolled a **{}** on a d{}", result, sides))
            .footer(serenity::CreateEmbedFooter::new(format!("Range: 1–{}", sides)))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// ask the ball something
#[poise::command(slash_command)]
pub async fn eightball(
    ctx: Context<'_>,
    #[description = "The question to ask"] question: String,
) -> Result<(), Error> {
    let responses = [
        ("It is certain.", 0x00FF88),
        ("It is decidedly so.", 0x00FF88),
        ("Without a doubt.", 0x00FF88),
        ("Yes — definitely.", 0x00FF88),
        ("You may rely on it.", 0x00FF88),
        ("As I see it, yes.", 0x00FF88),
        ("Most likely.", 0x00E5FF),
        ("Outlook good.", 0x00E5FF),
        ("Yes.", 0x00E5FF),
        ("Signs point to yes.", 0x00E5FF),
        ("Reply hazy, try again.", 0xFFAA00),
        ("Ask again later.", 0xFFAA00),
        ("Better not tell you now.", 0xFFAA00),
        ("Cannot predict now.", 0xFFAA00),
        ("Concentrate and ask again.", 0xFFAA00),
        ("Don't count on it.", 0xFF3B3B),
        ("My reply is no.", 0xFF3B3B),
        ("My sources say no.", 0xFF3B3B),
        ("Outlook not so good.", 0xFF3B3B),
        ("Very doubtful.", 0xFF3B3B),
    ];
    let (answer, color) = responses[rand::thread_rng().gen_range(0..responses.len())];

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🎱 Magic 8-Ball")
            .field("Question", &question, false)
            .field("Answer", answer, false)
            .footer(serenity::CreateEmbedFooter::new("The oracle has spoken"))
            .color(color),
    ))
    .await?;
    Ok(())
}

/// get a random programming joke
#[poise::command(slash_command)]
pub async fn joke(ctx: Context<'_>) -> Result<(), Error> {
    let jokes = [
        "Why do programmers prefer dark mode?\nBecause light attracts bugs.",
        "How many programmers does it take to change a light bulb?\nNone — that's a hardware problem.",
        "To understand recursion, you must first understand recursion.",
        "A programmer was told: \"Go to the store. Buy a loaf of bread. If they have eggs, get a dozen.\"\nHe returned with 12 loaves of bread.",
        "There are 10 types of people in the world: those who understand binary, and those who don't.",
        "Why did the programmer quit his job?\nBecause he didn't get arrays.",
        "What's a programmer's favorite place?\nThe foo bar.",
        "Why don't programmers like nature?\nToo many bugs and no documentation.",
        "A SQL query walks into a bar, walks up to two tables, and asks... 'Can I join you?'",
        "Why was the JavaScript developer sad?\nBecause he didn't Node how to Express himself.",
        "Why did the developer go broke?\nBecause he used up all his cache.",
        "What do you call a developer who doesn't comment their code?\nA sociopath.",
        "I would tell you a UDP joke, but you might not get it.",
        "Why was the Rust code so happy?\nBecause it had no dangling references.",
        "What's a programmer's favorite movie?\nNull Fiction.",
        "Why do Java developers wear glasses?\nBecause they can't C#.",
        "How do you know a developer is an extrovert?\nThey look at YOUR shoes when talking to you.",
        "A product manager walks into a bar and asks for a beer. Then for 2 beers. Then for 0. Then for 99999.\nThe programmer panics.",
        "Why did the programmer get kicked out of school?\nBecause he kept breaking the class.",
        "What do you call a snake that builds software?\nA Python developer.",
    ];
    let joke = jokes[rand::thread_rng().gen_range(0..jokes.len())];

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("😂 Programming Joke")
            .description(joke)
            .footer(serenity::CreateEmbedFooter::new("Powered by AegisForge Humor Engine v4"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// get a random interesting fact
#[poise::command(slash_command)]
pub async fn fact(ctx: Context<'_>) -> Result<(), Error> {
    let facts = [
        "A jiffy is an actual unit of time — 1/100th of a second.",
        "The first computer bug was a real bug: a moth found in a Harvard relay in 1947.",
        "There are over 700 programming languages in existence.",
        "The Apollo 11 guidance computer had less processing power than a modern toaster.",
        "Honey never spoils — edible honey has been found in 3,000-year-old Egyptian tombs.",
        "Octopuses have three hearts and blue blood.",
        "Rust was voted the most-loved programming language on Stack Overflow 8 years in a row.",
        "The word 'byte' was coined in 1956 to avoid accidental confusion with 'bit'.",
        "The first 1GB hard drive (1980) weighed 550 lbs and cost $40,000.",
        "Cleopatra lived closer in time to the Moon landing than to the construction of the Great Pyramid.",
        "Bananas are technically berries, but strawberries are not.",
        "The dot over the letter 'i' is called a tittle.",
        "A group of flamingos is called a flamboyance.",
        "The first emoji was created in 1999 by Shigetaka Kurita.",
        "Nintendo was founded in 1889 as a playing card company.",
        "Sharks are older than trees — they've existed for about 450 million years.",
        "A day on Venus is longer than a year on Venus.",
        "There are more possible games of chess than atoms in the observable universe.",
        "The 'https' in a URL stands for HyperText Transfer Protocol Secure. The 's' was added in 1994.",
        "The average person blinks about 15–20 times per minute, but only 3–8 times per minute while reading a screen.",
    ];
    let f = facts[rand::thread_rng().gen_range(0..facts.len())];

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("💡 Random Fact")
            .description(f)
            .footer(serenity::CreateEmbedFooter::new("AegisForge Knowledge Core"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// send a cute cat picture
#[poise::command(slash_command)]
pub async fn cat(ctx: Context<'_>) -> Result<(), Error> {
    #[derive(serde::Deserialize)]
    struct AnimalResponse { image: String, fact: String }

    let res = ctx.data().http_client
        .get("https://some-random-api.com/animal/cat")
        .send().await
        .and_then(|r| r.error_for_status())
        .map_err(|e| format!("Couldn't reach the cat API: {}", e))?
        .json::<AnimalResponse>().await
        .map_err(|e| format!("Couldn't parse the cat response: {}", e))?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🐱 Meow!")
            .image(res.image)
            .description(res.fact)
            .footer(serenity::CreateEmbedFooter::new("Powered by some-random-api.com"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// send a cute dog picture
#[poise::command(slash_command)]
pub async fn dog(ctx: Context<'_>) -> Result<(), Error> {
    // curated list of dog image URLs from dog.ceo (stable breed images)
    let dogs = [
        "https://images.dog.ceo/breeds/retriever-golden/n02099601_7771.jpg",
        "https://images.dog.ceo/breeds/husky/n02110185_1469.jpg",
        "https://images.dog.ceo/breeds/labrador/n02099712_3025.jpg",
        "https://images.dog.ceo/breeds/pomeranian/n02112018_1090.jpg",
        "https://images.dog.ceo/breeds/shiba/shiba-7.jpg",
        "https://images.dog.ceo/breeds/corgi-cardigan/n02113186_1030.jpg",
        "https://images.dog.ceo/breeds/samoyed/n02111889_4266.jpg",
        "https://images.dog.ceo/breeds/chow/n02112137_9985.jpg",
        "https://images.dog.ceo/breeds/malinois/n02105162_622.jpg",
        "https://images.dog.ceo/breeds/retriever-golden/n02099601_3004.jpg",
    ];
    let url = dogs[rand::thread_rng().gen_range(0..dogs.len())];
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🐶 Woof!")
            .image(url)
            .footer(serenity::CreateEmbedFooter::new("Powered by dog.ceo"))
            .color(0xD2691E),
    ))
    .await?;
    Ok(())
}

/// send a cute fox picture
#[poise::command(slash_command)]
pub async fn fox(ctx: Context<'_>) -> Result<(), Error> {
    // randomfox.ca hosts 123 numbered fox images
    let n = rand::thread_rng().gen_range(1..=123u32);
    let url = format!("https://randomfox.ca/images/{}.jpg", n);
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🦊 What does the fox say?")
            .image(url)
            .footer(serenity::CreateEmbedFooter::new("Powered by randomfox.ca"))
            .color(0xFF5722),
    ))
    .await?;
    Ok(())
}

/// send a cute panda picture
#[poise::command(slash_command)]
pub async fn panda(ctx: Context<'_>) -> Result<(), Error> {
    #[derive(serde::Deserialize)]
    struct AnimalResponse { image: String, fact: String }

    let res = ctx.data().http_client
        .get("https://some-random-api.com/animal/panda")
        .send().await
        .and_then(|r| r.error_for_status())
        .map_err(|e| format!("Couldn't reach the panda API: {}", e))?
        .json::<AnimalResponse>().await
        .map_err(|e| format!("Couldn't parse the panda response: {}", e))?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🐼 Panda Time!")
            .image(res.image)
            .description(res.fact)
            .footer(serenity::CreateEmbedFooter::new("Powered by some-random-api.com"))
            .color(0x111111),
    ))
    .await?;
    Ok(())
}

/// send a cute bird picture
#[poise::command(slash_command)]
pub async fn bird(ctx: Context<'_>) -> Result<(), Error> {
    #[derive(serde::Deserialize)]
    struct AnimalResponse { image: String, fact: String }

    let res = ctx.data().http_client
        .get("https://some-random-api.com/animal/bird")
        .send().await
        .and_then(|r| r.error_for_status())
        .map_err(|e| format!("Couldn't reach the bird API: {}", e))?
        .json::<AnimalResponse>().await
        .map_err(|e| format!("Couldn't parse the bird response: {}", e))?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🐦 Chirp!")
            .image(res.image)
            .description(res.fact)
            .footer(serenity::CreateEmbedFooter::new("Powered by some-random-api.com"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// give someone a virtual cookie
#[poise::command(slash_command)]
pub async fn cookie(
    ctx: Context<'_>,
    #[description = "The user to give a cookie to"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🍪 Cookie Delivery!")
            .description(format!(
                "**{}** slid a warm cookie across the table to **{}**! 🍪",
                ctx.author().name, user.name
            ))
            .color(0xD2691E),
    ))
    .await?;
    Ok(())
}

/// give someone a hug
#[poise::command(slash_command)]
pub async fn hug(
    ctx: Context<'_>,
    #[description = "The user to hug"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .description(format!(
                "🤗 **{}** wrapped **{}** in a big warm hug!",
                ctx.author().name, user.name
            ))
            .color(0xFF69B4),
    ))
    .await?;
    Ok(())
}

/// give someone a pat
#[poise::command(slash_command)]
pub async fn pat(
    ctx: Context<'_>,
    #[description = "The user to pat"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .description(format!(
                "🖐️ **{}** gave **{}** a gentle head pat!",
                ctx.author().name, user.name
            ))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// give someone a kiss
#[poise::command(slash_command)]
pub async fn kiss(
    ctx: Context<'_>,
    #[description = "The user to kiss"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .description(format!(
                "💋 **{}** gave **{}** a sweet kiss! 💕",
                ctx.author().name, user.name
            ))
            .color(0xFF0000),
    ))
    .await?;
    Ok(())
}

/// slap someone
#[poise::command(slash_command)]
pub async fn slap(
    ctx: Context<'_>,
    #[description = "The user to slap"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .description(format!(
                "💥 **{}** slapped **{}** with full force! OUCH.",
                ctx.author().name, user.name
            ))
            .color(0xFF5722),
    ))
    .await?;
    Ok(())
}

/// get a random meme image
#[poise::command(slash_command)]
pub async fn meme(ctx: Context<'_>) -> Result<(), Error> {
    // curated stable meme image URLs
    let memes = [
        ("This Is Fine", "https://i.kym-cdn.com/entries/icons/original/000/018/012/this_is_fine.jpeg"),
        ("Drake Approves", "https://i.kym-cdn.com/entries/icons/original/000/019/490/dd2.jpg"),
        ("Distracted Boyfriend", "https://i.kym-cdn.com/photos/images/newsfeed/001/525/547/0b3.jpg"),
        ("Expanding Brain", "https://i.kym-cdn.com/photos/images/newsfeed/001/035/474/04a.jpg"),
        ("Surprised Pikachu", "https://i.kym-cdn.com/photos/images/newsfeed/001/480/179/07e.jpg"),
        ("Woman Yelling at Cat", "https://i.kym-cdn.com/photos/images/newsfeed/001/536/075/c40.jpg"),
        ("Two Buttons", "https://i.kym-cdn.com/photos/images/newsfeed/001/070/617/bec.jpg"),
    ];
    let (title, url) = memes[rand::thread_rng().gen_range(0..memes.len())];
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("🎭 {}", title))
            .image(url)
            .footer(serenity::CreateEmbedFooter::new("Fresh from the meme vault"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// check the compatibility between two users
#[poise::command(slash_command)]
pub async fn ship(
    ctx: Context<'_>,
    #[description = "First user"] user1: serenity::User,
    #[description = "Second user (defaults to you)"] user2: Option<serenity::User>,
) -> Result<(), Error> {
    let u2 = user2.as_ref().unwrap_or_else(|| ctx.author());
    let percent: u32 = rand::thread_rng().gen_range(0..=100);
    let filled = (percent / 10) as usize;
    let bar = format!(
        "{}{} {}%",
        "❤️".repeat(filled),
        "🖤".repeat(10 - filled),
        percent
    );
    let verdict = match percent {
        0..=20 => "Not a chance. ❄️",
        21..=40 => "Maybe as friends? 🤝",
        41..=60 => "There's some chemistry! 🧪",
        61..=80 => "Looking pretty good! 🔥",
        81..=99 => "A perfect match! ❤️",
        _ => "Soulmates for eternity! 💍",
    };
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("💖 Matchmaking Forge")
            .description(format!("Checking compatibility between **{}** and **{}**...", user1.name, u2.name))
            .field("Compatibility", bar, false)
            .field("Verdict", verdict, false)
            .footer(serenity::CreateEmbedFooter::new("Results are 100% scientifically accurate"))
            .color(0xFF69B4),
    ))
    .await?;
    Ok(())
}

/// rate something out of 10
#[poise::command(slash_command)]
pub async fn rate(
    ctx: Context<'_>,
    #[description = "What to rate"] thing: String,
) -> Result<(), Error> {
    let rating: u32 = rand::thread_rng().gen_range(0..=10);
    let comment = match rating {
        0 => "Absolutely terrible. No notes.",
        1..=3 => "Pretty rough. Could be worse.",
        4..=6 => "Decent. Room for improvement.",
        7..=8 => "Pretty solid, actually.",
        9 => "Exceptional work. Chef's kiss.",
        _ => "A perfect 10. Genuinely flawless.",
    };
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("⚖️ The Oracle's Rating")
            .description(format!("**{}** gets a **{}/10**", thing, rating))
            .field("Verdict", comment, false)
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// mock text in sPoNgEbOb CaSe
#[poise::command(slash_command)]
pub async fn mock(
    ctx: Context<'_>,
    #[description = "The text to mock"] text: String,
) -> Result<(), Error> {
    let mocked: String = text
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect();
    ctx.say(mocked).await?;
    Ok(())
}

/// reverse some text
#[poise::command(slash_command)]
pub async fn reverse(
    ctx: Context<'_>,
    #[description = "The text to reverse"] text: String,
) -> Result<(), Error> {
    let reversed: String = text.chars().rev().collect();
    ctx.say(reversed).await?;
    Ok(())
}

/// convert text to spaced-out ASCII display
#[poise::command(slash_command)]
pub async fn ascii(
    ctx: Context<'_>,
    #[description = "The text to convert (max 10 chars)"] text: String,
) -> Result<(), Error> {
    if text.len() > 10 {
        return Err("Text must be 10 characters or fewer.".into());
    }
    let spaced: String = text
        .to_uppercase()
        .chars()
        .map(|c| format!(" {} ", c))
        .collect::<Vec<_>>()
        .join("");
    ctx.say(format!("```\n{}\n```", spaced.trim())).await?;
    Ok(())
}

/// randomly choose between options (separate with commas or spaces)
#[poise::command(slash_command)]
pub async fn choose(
    ctx: Context<'_>,
    #[description = "Options to pick from, separated by commas (e.g. pizza, sushi, tacos)"]
    options: String,
) -> Result<(), Error> {
    let choices: Vec<&str> = options
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if choices.len() < 2 {
        return Err("Please provide at least 2 options separated by commas.".into());
    }

    let chosen = choices[rand::thread_rng().gen_range(0..choices.len())];

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🎯 AegisForge Decides")
            .description(format!("Out of **{}** options, the forge chose:", choices.len()))
            .field("Winner", format!("**{}**", chosen), false)
            .footer(serenity::CreateEmbedFooter::new("Cryptographically random™"))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// get a random trivia question
#[poise::command(slash_command)]
pub async fn trivia(ctx: Context<'_>) -> Result<(), Error> {
    let questions = [
        ("What is the capital of Australia?", "Canberra", vec!["Sydney", "Melbourne", "Brisbane"]),
        ("How many bits are in a byte?", "8", vec!["4", "16", "32"]),
        ("Which planet is closest to the Sun?", "Mercury", vec!["Venus", "Earth", "Mars"]),
        ("What year was the first iPhone released?", "2007", vec!["2005", "2008", "2010"]),
        ("What programming language is Rust's borrow checker written in?", "Rust", vec!["C++", "Haskell", "Python"]),
        ("What does HTML stand for?", "HyperText Markup Language", vec!["High-Tech Modern Language", "HyperText Modeling Language", "Home Tool Markup Language"]),
        ("How many colors are in a rainbow?", "7", vec!["5", "6", "8"]),
        ("What is the chemical symbol for gold?", "Au", vec!["Go", "Gd", "Ag"]),
        ("Which country invented the Internet?", "United States", vec!["United Kingdom", "Germany", "Japan"]),
        ("What is 7 × 8?", "56", vec!["54", "58", "48"]),
        ("Which company created the Rust programming language?", "Mozilla", vec!["Google", "Microsoft", "Apple"]),
        ("What does SQL stand for?", "Structured Query Language", vec!["Simple Query Language", "Sequential Query Logic", "Standard Query List"]),
        ("How many planets are in the Solar System?", "8", vec!["7", "9", "10"]),
        ("What is the largest ocean on Earth?", "Pacific", vec!["Atlantic", "Indian", "Arctic"]),
        ("In which year did the Berlin Wall fall?", "1989", vec!["1987", "1991", "1993"]),
        ("What does RAM stand for?", "Random Access Memory", vec!["Read-only Access Memory", "Rapid Array Module", "Runtime Address Map"]),
        ("Which gas do plants absorb from the atmosphere?", "Carbon dioxide", vec!["Oxygen", "Nitrogen", "Hydrogen"]),
        ("What is the smallest prime number?", "2", vec!["0", "1", "3"]),
        ("Who wrote the Harry Potter series?", "J.K. Rowling", vec!["J.R.R. Tolkien", "C.S. Lewis", "Stephen King"]),
        ("What does HTTP stand for?", "HyperText Transfer Protocol", vec!["High-Throughput Transfer Protocol", "HyperText Transport Process", "Hyperlink Text Transfer Protocol"]),
    ];

    let (question, answer, mut wrong) = questions[rand::thread_rng().gen_range(0..questions.len())].clone();

    let mut all: Vec<&str> = vec![answer];
    all.append(&mut wrong);
    // shuffle options
    for i in (1..all.len()).rev() {
        let j = rand::thread_rng().gen_range(0..=i);
        all.swap(i, j);
    }

    let letters = ["🇦", "🇧", "🇨", "🇩"];
    let options_display: String = all
        .iter()
        .enumerate()
        .map(|(i, opt)| format!("{} {}", letters[i], opt))
        .collect::<Vec<_>>()
        .join("\n");

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🧠 AegisForge Trivia")
            .description(format!("**{}**", question))
            .field("Options", &options_display, false)
            .field("Answer", format!("||{}||", answer), false)
            .footer(serenity::CreateEmbedFooter::new("Answer is hidden — click to reveal!"))
            .color(0xBF5AF2),
    ))
    .await?;
    Ok(())
}

/// roast someone (all in good fun)
#[poise::command(slash_command)]
pub async fn roast(
    ctx: Context<'_>,
    #[description = "The user to roast"] user: serenity::User,
) -> Result<(), Error> {
    let roasts = [
        "is the human equivalent of a participation trophy.",
        "has the personality of a lukewarm cup of water.",
        "is like a software update — when you see them, you think 'not now'.",
        "is the reason we have warning labels on everything.",
        "could start a fight in an empty room.",
        "types with two fingers and still makes typos.",
        "is the loading screen nobody asked for.",
        "peaked in a server nobody remembers.",
        "has a face only a CAPTCHA could love.",
        "is like a NaN — not a number, not a person, just an error.",
    ];
    let roast = roasts[rand::thread_rng().gen_range(0..roasts.len())];

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔥 Roast Session")
            .description(format!("**{}** {}", user.name, roast))
            .footer(serenity::CreateEmbedFooter::new("Requested by the forge • All in good fun"))
            .color(0xFF5722),
    ))
    .await?;
    Ok(())
}

/// give someone a genuine compliment
#[poise::command(slash_command)]
pub async fn compliment(
    ctx: Context<'_>,
    #[description = "The user to compliment"] user: serenity::User,
) -> Result<(), Error> {
    let compliments = [
        "is genuinely one of the most reliable people in this server. 🌟",
        "has a great sense of humor and makes every conversation better. 😄",
        "is the kind of person who makes a community actually worth being in. 💎",
        "has incredible taste. Seriously impressive. ✨",
        "is underrated. People should talk about how great they are more often. 🚀",
        "brings energy to this server that can't be replaced. 🔥",
        "is smarter than they give themselves credit for. 🧠",
        "has a way of making people feel welcome without even trying. 🤝",
        "is proof that good people still exist on the internet. 💙",
        "is an absolute legend in the making. The forge approves. 🛡️",
    ];
    let compliment = compliments[rand::thread_rng().gen_range(0..compliments.len())];

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("💙 A Genuine Compliment")
            .description(format!("**{}** {}", user.name, compliment))
            .footer(serenity::CreateEmbedFooter::new("Spread positivity • AegisForge"))
            .color(0x00FF88),
    ))
    .await?;
    Ok(())
}


