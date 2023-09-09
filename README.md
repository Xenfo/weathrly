# weathrly

Weather utilities (today's forecast and forecast predictions) made for my CS50x final project.

### Video Demo: https://youtu.be/arjHjG8wX2g
### Description:

This project is a utility for getting today's weather or predictions for the future right in your terminal. I got the idea from my upcoming trip to Italy in March, 2024, where I wanted to ballpark the temperatures that I might experience on my trip. I then expanded it to include today's weather.

Starting off, `src/main.rs` is the entry point of the application. It handles parsing the arguments passed to the program and executing the right command.

Next, `src/api.rs` contains the API responses for calling [Open Meteo](https://open-meteo.com/), which provides historical weather data, today's forecast and geocoding information for free.

`src/commands/mod.rs` contains the commands that the program supports.
- `src/commands/predict.rs` contains the implementation of the predict command. It makes a call to the geocoding API to get the coordinates of the city passed in. Then it calls the historical forecast API with 3 past years worth of data from today. I then take those results and get the median of all the properties I requested so that the resulting properties are realistic. Then I print it.
- `src/commands/today.rs` contains the implementation of the today command. Similarly to predict, I call the geocoding API to get the coordinates of the city. Then I get the daily forecast and clean up the data so it looks nice when printed in a table.

I could have used a climate change model instead of historical data for the predict command but comparing the results, the historical data seemed to be closer to what is being seen these past years. The historical data implementation also required more effort and thinking to implement, which made it more enjoyable for me. I also chose to limit the predictions to just temperature and apparent temperature because the other meausurements don't translate accross the years as well as these two.
