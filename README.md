# weathrly

Weather utilities (today's forecast and forecast predictions) made for my CS50x final project.

## Usage

```sh
weathrly --help
```

## CS50x

### Video Demo: https://youtu.be/arjHjG8wX2g

### Description:

This project is a utility for getting today's weather or predictions for the future right in your terminal. I got the idea from my upcoming trip to Italy in March, 2024, where I wanted to ballpark the temperatures that I might experience on my trip. I later expanded it to include support for getting today's weather forecast.

Starting off, `src/main.rs` is the entry point of the application. It handles parsing the arguments passed to the program and executing the right command. I'm using the Clap CLI framework to automatically handle parsing the arguments and creating utility commands like help and version.

Next, `src/api.rs` contains the API responses for calling [Open Meteo](https://open-meteo.com/), which provides historical weather data, today's forecast and geocoding information for free. The file contains many structs which are the JSON objects returned by the API and one enum which maps numerical weather codes to their string counter part. It was difficult for me to deserialize the dates since Serde didn't like the format that the API returned, so I ended up keeping it as a string and parsing the date (specifically the time) myself.

`src/commands/mod.rs` contains the commands that the program supports in enum form. It's used in a match case in `src/main.rs`.
- `src/commands/predict.rs` contains the implementation of the `predict` command. It makes a call to the geocoding API to get the coordinates of the city passed in. Then it calls the historical forecast API with 3 past years worth of data from today. I then take those results and get the median of all the properties I requested so that the resulting properties are realistic. Then I print it.
- `src/commands/today.rs` contains the implementation of the `today` command. Similarly to `predict`, I call the geocoding API to get the coordinates of the city. Then I get the daily forecast and clean up the data so it looks nice when printed in a table.

I could have used a climate change model instead of historical data for the `predict` command but comparing the results, the historical data seemed to be closer to what is being seen these past years. The historical data implementation also required more effort and thinking to implement, which made it more enjoyable for me. I also chose to limit the predictions to just temperature and apparent temperature because the other meausurements don't translate accross the years as well as these two.
