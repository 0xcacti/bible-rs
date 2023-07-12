## bible-rs

bible-rs is a verse of the day Bible CLI tool written in rust.  I am a brand new 
rust developer, so please tear apart the code that I have written if you see fit 
to do so.  

The tool currently allows you to do the following

  list    Get a list of Books in the provided Bible version
  daily   Get the daily random verse from the Bible
  new     Get a new random verse from the Bible
  book    Get a random verse from a specific book of the Bible
  bibles  Get available Bible versions


Bible verses and transalations are pulled from this [Bible API](https://docs.api.bible/)

You can configure your api_key and version via the bible-rs.toml file, environment variable 
or CLI flag.  



