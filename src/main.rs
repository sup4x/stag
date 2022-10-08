use clap::Parser;
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Quote
    #[clap(short, long, value_parser)]
    quote: Option<String>,
    // Tag
    #[clap(short, long, value_parser)]
    tag: Option<String>,
    // Author
    #[clap(short, long, value_parser)]
    author: Option<String>,
    // Rating
    #[clap(short, long, value_parser)]
    rating: Option<u8>,
}

#[derive(Debug)]
struct RR {
    text: String,
    name: String,
    tags: String,
}

fn main() -> Result<()> {
     let conn = Connection::open("test.db")?;

    let mut stmt = conn.prepare(
        "select quotes.text as text, a.name as name, group_concat( t.name ) as tags
            from quotes
            left join quote_tag qt on quotes.code = qt.quote_code
            left join tags t on qt.tag_code = t.code
            left join authors a on quotes.author_code = a.code
            group by quotes.code;",
    )?;

    let cats = stmt.query_map([], |row| {
        Ok(RR {
            text: row.get(0)?,
            name: row.get(1)?,
            tags: row.get(2)?
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
    //  let args = Args::parse();
    //  match (args.quote, args.tag, args.author, args.rating) {
    //     (Some(quote), Some(tag), Some(author), Some(rating)) => {
    //         println!("{}", quote);
    //         println!("{}", tag);
    //         println!("{}", author);
    //         println!("{}", rating);
    //     },
    //      _ => {}
    // }
}
