use anyhow::Error;
use clap::Parser;
use fred::prelude::Error as FredError;
use fred::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'u', long, default_value = "redis://localhost")]
    redis_url: String,
    #[arg(short = 'c', long, default_value = "hack4rail")]
    channel: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let config = Config::from_url(&args.redis_url)?;
    let subscriber = Builder::from_config(config).build()?;
    subscriber.init().await?;
    subscriber.subscribe(args.channel).await?;

    let message_task = subscriber.on_message(|message| async move {
        println!(
            "{}: {}",
            message.channel,
            message.value.as_str().unwrap_or("".into())
        );
        Ok::<_, FredError>(())
    });

    message_task.await??;

    Ok(())
}
