# am-you-unique
A very bad clone of [AmIUnique](https://amiunique.org), [Cover Your Tracks](https://coveryourtracks.eff.org), and others like them. This version only uses user agents to identify your browser.  

## Run
If you want to run it by yourself for some reason, you need to have Cargo (preferably the latest version, not sure which ones this supports) and an account on Back4App. Once you've created an account and project on Back4App, set the environment variable `BACK4APP_APP_ID` to the app id and `BACK4APP_API_KEY` to the REST API key. After you've set the environment variables, type `cargo run` to start the server. Visit `http://localhost:8080` to see the site.
