# Elasticsearch-Parser
File parser that converts text documents into importable elastic search JSON strings

**WORK IN PROCESS**

Woke up at 3AM last night (4.3.2022) thinking about ideas on how to import my bulk data (3TB) into a nice and fast search engine with little effort whilst learning a new language (Rust) and came up with an idea:

Usually developers have to deal with data that is retained in different formats. This naturally means that you can't treat the files the same way when parsing them. 
This means, that you will spend A LOT of time parsing different data sets.

I'm too lazy to do this myself and I feel like it would be a huge waste of time so this project is an attempt to almost fully automate the process. (Only having to specify the format once)

Elasticsearch-Parser turns a text file with data delimited with a character and turns it into JSON files containing JSON Strings that you can immediately import into elastic search.


