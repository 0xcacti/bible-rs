## Welcome to bible-rs 

I am working on learning rust.  I want to improve my skills.  The plan here is 
to write a CLI tool that can return a random Bible verse each day.  I want to add
another setting that lets you get a verse from a specified book as well.  


Okay, this is turning into a nice small project. I am really struggling with 
just getting organized and feel bogged down with the rust, so I am going to write 
out what I want here to make it easier for me to build it. 

Goals: 

I want users to be able to specify their configuration using defaults, 
and override them with some configuration file or command line arguments. 
This is a little bit ridiculous since there are only two things that need
configuration, the api key and the default version of the Bible from which
to pull verses, however, I want to do it for good practice.  

The priority should be 
defaults < config < flags 

I am not sure the best way to do this, I think what I will do is have 
defaults hard coded in, allow a config.toml file, and then override with flags


