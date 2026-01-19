# Realization
## What this bot should be able to do
This bot should be able to send a random cat picture to the user. 

## User flow
1. User sends a message to the bot.
2. Bot sends a random cat picture to the user.

## User stories
1. User sends a message to the bot.
2. Bot sends a random cat picture to the user.
3. User can tag bot in the group and get a random picture without adding bot to the group.

## Technical details
1. Bot uses the Cat API to get a random cat picture.
2. Bot uses the Telegram API to send the picture to the user.
3. Bot uses the dotenvy crate to load the token from the .env file.

## Modules
- API request module
- Main Telegram module