use std::time::{
    Instant
};
use serenity::{
  framework::standard::{
      CommandResult,
      macros::command,
      Args
  },
  model::prelude::*,
  prelude::*
};
use colour::{
    magenta_ln
};
use wikipedia;
use titlecase::titlecase;

#[command]
async fn wiki(ctx: &Context, msg: &Message, search_query: Args) -> CommandResult {
    let query = search_query.rest();

    if query.chars().count() < 1 {
        msg.reply_ping(ctx, "You're missing the search query").await?;
        return Ok(());
    }

    let mut reply = msg.reply_ping(ctx, "Scraping...").await?;

    let start = Instant::now();

    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.page_from_title(query.to_owned());

    let duration = start.elapsed();

    if let Err(why) = page.get_summary() {
        reply.edit(ctx, |c| c.content(format!("{:?}: couldn't retrieve results from that wiki page. Either it doesn't exist, or you need to be more specific.", why))).await?;
        return Ok(());
    };

    let title = titlecase(&page.get_title().unwrap());
    let mut summary = page.get_summary().unwrap();
    let url =  "https://en.wikipedia.org/wiki/".to_owned() + &title.replace(" ", "_");

    if summary.len() > 500 {
        summary = format!("{}...", &summary[0..497]);
    };

    reply.edit(ctx, |c| {
        c.content(format!("Scraped in `{:?}`", duration));
        c.embed(|e| {
            e.title(title);
            e.description(summary);
            e.colour(0xb7410e);
            e.url(url);
            e
        })
    }).await?;

    magenta_ln!("Wiki search made for: {} by: {}", titlecase(&page.get_title().unwrap()), msg.author.name);

    Ok(())
}