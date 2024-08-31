# dnd-discord-bot
A Discord bot to automate tedious DND activities 😄
See the discord-example-app repo for the basis of this code: https://github.com/discord/discord-example-app.


## Build

I would highly recommend following the [Building your first Discord app](https://discord.com/developers/docs/quick-start/getting-started) instructions to get a Discord app set up. You will need to insert these credentials to a `.env` file - rename the `.env.sample` file to `.env`, and replace `<YOUR_APP_ID>`, `<YOUR_BOT_TOKEN>`, and `<YOUR_PUBLIC_KEY>`.

The easiest way to build out this repo is using Docker:
```
docker build . -t bot
```
To run the app locally, you can use the provided, simple docker-compose file:
```
docker compose up -d
```
The app will listen on port 3000. At this point, you need the app to have a publically accessible endpoint (i.e. port 3000 needs to be reachable via the internet). This depends on where you will be hosting this application - a reverse proxy on a host with a DNS entry will work, or ngrok can be used as well.

Once your app is reachable on the internet, follow the [Building your first Discord app](https://discord.com/developers/docs/quick-start/getting-started) instructions to get started on creating an app, and add your url to the Interaction Endpoint URL configuration, and append `/interactions` to that url (since that is the endpoint the app is listening on).