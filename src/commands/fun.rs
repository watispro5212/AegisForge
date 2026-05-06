use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::Rng;

/// Fun and social commands
#[poise::command(
    slash_command,
    subcommands("coinflip", "dice", "eightball", "joke", "fact", "cat", "cookie", "hug", "pat", "kiss", "slap"),
    category = "Fun"
)]
pub async fn fun(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Flip a coin
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

/// Roll a dice
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

/// Ask the Magic 8-Ball a question
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
