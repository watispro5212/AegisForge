use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::Rng;

/// fun stuff
#[poise::command(
    slash_command,
    subcommands("coinflip", "dice", "eightball", "joke", "fact", "cat", "dog", "fox", "panda", "bird", "cookie", "hug", "pat", "kiss", "slap", "meme", "ship", "rate", "mock", "reverse", "owo", "ascii"),
    category = "fun"
)]
pub async fn fun(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// flip a coin
#[poise::command(slash_command)]
pub async fn coinflip(ctx: Context<'_>) -> Result<(), Error> {
    let result = if rand::random::<bool>() { "Heads" } else { "Tails" };
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🪙 Coin Flip")
            .description(format!("The coin landed on **{}**!", result))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// roll some dice
#[poise::command(slash_command)]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "Number of sides (defaults to 6)"] sides: Option<u32>,
) -> Result<(), Error> {
    let sides = sides.unwrap_or(6).max(2);
    let result = (rand::thread_rng().gen_range(0..sides)) + 1;
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🎲 Dice Roll")
            .description(format!("You rolled a **{}** (1-{})", result, sides))
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// ask the 8ball something
#[poise::command(slash_command)]
pub async fn eightball(
    ctx: Context<'_>,
    #[description = "The question to ask"] question: String,
) -> Result<(), Error> {
    let responses = [
        "It is certain.", "It is decidedly so.", "Without a doubt.", "Yes - definitely.",
        "You may rely on it.", "As I see it, yes.", "Most likely.", "Outlook good.",
        "Yes.", "Signs point to yes.", "Reply hazy, try again.", "Ask again later.",
        "Better not tell you now.", "Cannot predict now.", "Concentrate and ask again.",
        "Don't count on it.", "My reply is no.", "My sources say no.", "Outlook not so good.",
        "Very doubtful."
    ];
    let index = rand::thread_rng().gen_range(0..responses.len());
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🎱 Magic 8-Ball")
            .field("Question", question, false)
            .field("Answer", responses[index], false)
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// Get a random programming joke
#[poise::command(slash_command)]
pub async fn joke(ctx: Context<'_>) -> Result<(), Error> {
    let jokes = [
        "Why do programmers prefer dark mode?\nBecause light attracts bugs.",
        "How many programmers does it take to change a light bulb?\nNone, that's a hardware problem.",
        "To understand what recursion is, you must first understand recursion.",
        "A programmer is told to \"go to the store and buy a loaf of bread. If they have eggs, get a dozen.\"\nHe returns with 12 loaves of bread.",
        "There are 10 types of people in the world: those who understand binary, and those who don't.",
        "Why did the programmer quit his job?\nBecause he didn't get arrays."
    ];
    let index = rand::thread_rng().gen_range(0..jokes.len());
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("😂 Programming Joke")
            .description(jokes[index])
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// Get a random interesting fact
#[poise::command(slash_command)]
pub async fn fact(ctx: Context<'_>) -> Result<(), Error> {
    let facts = [
        "A jiffy is an actual unit of time: 1/100th of a second.",
        "The first computer bug was an actual real bug: a moth.",
        "There are over 700 programming languages in the world.",
        "The Apollo 11 guidance computer had less processing power than a modern smartphone charger.",
        "Honey never spoils. Archaeologists have found pots of honey in ancient Egyptian tombs that are over 3,000 years old.",
        "Octopuses have three hearts."
    ];
    let index = rand::thread_rng().gen_range(0..facts.len());
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("💡 Random Fact")
            .description(facts[index])
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// Send a cute cat picture
#[poise::command(slash_command)]
pub async fn cat(ctx: Context<'_>) -> Result<(), Error> {
    let cat_url = format!("https://cataas.com/cat?width=500&height=500&cache={}", rand::random::<u32>());
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🐱 Meow!")
            .image(cat_url)
            .color(0x00E5FF),
    ))
    .await?;
    Ok(())
}

/// Give someone a virtual cookie
#[poise::command(slash_command)]
pub async fn cookie(
    ctx: Context<'_>,
    #[description = "The user to give a cookie to"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🍪 Cookie Delivery!")
            .description(format!("{} gave a cookie to **{}**!", ctx.author().name, user.name))
            .color(0xD2691E),
    )).await?;
    Ok(())
}

/// Give someone a hug
#[poise::command(slash_command)]
pub async fn hug(
    ctx: Context<'_>,
    #[description = "The user to hug"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .description(format!("🤗 **{}** gave **{}** a big warm hug!", ctx.author().name, user.name))
            .color(0xFF69B4),
    )).await?;
    Ok(())
}

/// Give someone a pat
#[poise::command(slash_command)]
pub async fn pat(
    ctx: Context<'_>,
    #[description = "The user to pat"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .description(format!("🖐️ **{}** patted **{}** on the head!", ctx.author().name, user.name))
            .color(0x00E5FF),
    )).await?;
    Ok(())
}

/// Give someone a kiss
#[poise::command(slash_command)]
pub async fn kiss(
    ctx: Context<'_>,
    #[description = "The user to kiss"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .description(format!("💋 **{}** gave **{}** a sweet kiss!", ctx.author().name, user.name))
            .color(0xFF0000),
    )).await?;
    Ok(())
}

/// Slap someone
#[poise::command(slash_command)]
pub async fn slap(
    ctx: Context<'_>,
    #[description = "The user to slap"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .description(format!("💥 **{}** slapped **{}**! Ouch.", ctx.author().name, user.name))
            .color(0xFF5722),
    )).await?;
    Ok(())
}

/// Send a cute dog picture
#[poise::command(slash_command)]
pub async fn dog(ctx: Context<'_>) -> Result<(), Error> {
    let dog_url = format!("https://dog.ceo/api/breeds/image/random?cache={}", rand::random::<u32>());
    // Note: In a real app we'd fetch the JSON, but for this mock we'll just link a common one
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🐶 Woof!")
            .image("https://images.dog.ceo/breeds/pomeranian/n02112018_1090.jpg")
            .color(0x00E5FF),
    )).await?;
    Ok(())
}

/// Send a cute fox picture
#[poise::command(slash_command)]
pub async fn fox(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🦊 What does the fox say?")
            .image("https://randomfox.ca/images/1.jpg")
            .color(0xFF5722),
    )).await?;
    Ok(())
}

/// Send a cute panda picture
#[poise::command(slash_command)]
pub async fn panda(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🐼 Panda!")
            .image("https://some-random-api.com/img/panda")
            .color(0xFFFFFF),
    )).await?;
    Ok(())
}

/// Send a cute bird picture
#[poise::command(slash_command)]
pub async fn bird(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🐦 Chirp!")
            .image("https://some-random-api.com/img/bird")
            .color(0x00E5FF),
    )).await?;
    Ok(())
}

/// Get a random meme
#[poise::command(slash_command)]
pub async fn meme(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🎭 Fresh Meme")
            .image("https://meme-api.com/gimme")
            .color(0x00E5FF),
    )).await?;
    Ok(())
}

/// See the compatibility between two users
#[poise::command(slash_command)]
pub async fn ship(
    ctx: Context<'_>,
    #[description = "First user"] user1: serenity::User,
    #[description = "Second user"] user2: Option<serenity::User>,
) -> Result<(), Error> {
    let u1 = &user1;
    let u2 = user2.as_ref().unwrap_or_else(|| ctx.author());
    
    let percent = rand::thread_rng().gen_range(0..=100);
    let bar_filled = percent / 10;
    let bar = format!("{}{} ({}%)", "❤️".repeat(bar_filled as usize), "🖤".repeat((10 - bar_filled) as usize), percent);
    
    let comment = match percent {
        0..=20 => "Not a chance. ❄️",
        21..=40 => "Maybe as friends? 🤝",
        41..=60 => "There's some chemistry! 🧪",
        61..=80 => "Looking good! 🔥",
        81..=99 => "A perfect match! ❤️",
        _ => "Soulmates for eternity! 💍",
    };

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("💖 Matchmaking Forge")
            .description(format!("Checking compatibility for **{}** and **{}**...", u1.name, u2.name))
            .field("Result", bar, false)
            .field("Verdict", comment, false)
            .color(0xFF69B4),
    )).await?;
    Ok(())
}

/// Rate something
#[poise::command(slash_command)]
pub async fn rate(
    ctx: Context<'_>,
    #[description = "What to rate"] thing: String,
) -> Result<(), Error> {
    let rating = rand::thread_rng().gen_range(0..=10);
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("⚖️ The Oracle's Rating")
            .description(format!("I would rate **{}** a solid **{}/10**!", thing, rating))
            .color(0x00E5FF),
    )).await?;
    Ok(())
}

/// Mock some text (sPoNgEbOb CaSe)
#[poise::command(slash_command)]
pub async fn mock(
    ctx: Context<'_>,
    #[description = "The text to mock"] text: String,
) -> Result<(), Error> {
    let mocked: String = text.chars().enumerate().map(|(i, c)| {
        if i % 2 == 0 { c.to_lowercase().to_string() } else { c.to_uppercase().to_string() }
    }).collect();
    
    ctx.say(mocked).await?;
    Ok(())
}

/// Reverse some text
#[poise::command(slash_command)]
pub async fn reverse(
    ctx: Context<'_>,
    #[description = "The text to reverse"] text: String,
) -> Result<(), Error> {
    let reversed: String = text.chars().rev().collect();
    ctx.say(reversed).await?;
    Ok(())
}

/// owoify some text
#[poise::command(slash_command)]
pub async fn owo(
    ctx: Context<'_>,
    #[description = "The text to owoify"] text: String,
) -> Result<(), Error> {
    let owoified = text.replace("r", "w").replace("l", "w").replace("R", "W").replace("L", "W") + " uwu";
    ctx.say(owoified).await?;
    Ok(())
}

/// Convert text to large ASCII blocks (simple mock)
#[poise::command(slash_command)]
pub async fn ascii(
    ctx: Context<'_>,
    #[description = "The text to convert"] text: String,
) -> Result<(), Error> {
    if text.len() > 10 {
        return Err("Text too long for ASCII conversion (max 10 chars).".into());
    }
    
    let mut result = String::from("```\n");
    for c in text.to_uppercase().chars() {
        result.push_str(&format!(" {} ", c));
    }
    result.push_str("\n```");
    
    ctx.say(result).await?;
    Ok(())
}
