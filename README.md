# BilkentMeals_rs
Unofficial rust api for official bilkent meals [website](http://kafemud.bilkent.edu.tr/monu_tr.html). 
This api has only single function that returns the vector of daily meals.

# Dependenices
* chrono - For Date and DateTimes
* reqwest - To make Http GET request
* scraper - To scrap data from website
* encoding - For some reason website in not UTF-8 encoded so we need an encoding library

# Testing
`cargo test -- --nocapture`
If you see "success" with bunch of output meal names that means it works.

# Usage
You have two options here. 

##1.
Open cargo.toml file and add this line under dependenices
```
bilkentmeals = { git = "https://github.com/Furyzer0/BilkentMeals_rs"}
```

##2. 
Manually download the project and open cargo.toml file again
```
bilkentmeals = { path = "{The path of the dependency}" }
```
