# Chains

**(Rusty) Chains** was written in [**Rust**](https://www.rust-lang.org/) using the [**serenity**](https://docs.rs/serenity/latest/serenity/index.html) crate!

**Chains** primarily focuses on easy to set up, easy to use, and highly functional moderation and server logging tools.

## **Features to add** in order of priority

- [x] **Snapshot** command
    - [x] Snapshot chat between two messages
        - [x] Allow for both orientations of first, last and last, first when taking arguments.
- [ ] **Watch** command
    - [x] Log messages.
    - [ ] Log message edits.
    - [ ] Log username/nickname changes.
    - [ ] Log avatar edits.
- [ ] **Mirror** command
    - [ ] Log a whole channel by *mirroring* it into another channel.
- [ ] **Trigger** command
    - [ ] Delete OR log messages based on regex or text
        - [ ] Trigger database using a guild ID
- [ ] **Warn** command
    - [ ] Punishment configuration
        - [ ] Different punishment for an n<sup><sub>th</sub></sup> warn.
- [ ] **Halt** comand
    - [ ] Timeout user and delete their last n messages.
- **Miscellaneous** goals
    - [x] Channel configuration
        - [x] Persistent storage of configuration.
        - [ ] Allow more configurable channels
            - [ ] Different channels for different **Watch**ed users
            - [ ] Different channels for different **Trigger**s
            - [ ] Allow to specify channels on **Snapshot**
    - [x] Argument parsing
        - [x] Generally allow for any identifying data for an argument interchangeably
            - [x] Message links and IDs are interchangeable
            - [x] User mentions and IDs are interchangeable
            - [x] Channel mention, links and IDs are interchangeable
    - [ ] Add commands and how to use them to the README
    
---
<img src="http://www.apkmirror.com/wp-content/uploads/2016/07/577d7444b1370.png" alt="Discord" width="20"/> <sup>**pepsalt#1662** | **Salivala#1787** | **Holdank#7589** | **Anthony Fuller#1767** :) <3 </sup> 