# Sakata Bot

### Pre-requisites
- Rust
- AWS S3 Bucket

### Environment Variables
| Variable              | Example                                                     |
| --------------------- | ----------------------------------------------------------- |
| DISCORD_TOKEN         | db27e967adef493285a2a.37c452ba96edb2.7e967adef493285a2a37c4 |
| SAKATA_API_URL        | http://localhost:8080/                                      |
| AWS_ACCESS_KEY_ID     | AKIATYJPK7VM5RGASERT                                        |
| AWS_SECRET_ACCESS_KEY | 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                    |
| AWS_S3_BUCKET_NAME    | sakata-cards                                                |
| AWS_REGION            | us-east-2                                                   |

### Setup
- Running the Sakata Bot
```
> cargo run
```

### Add Sakata Bot to the discord server

Instructions to create and add discord bots [here](https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/)