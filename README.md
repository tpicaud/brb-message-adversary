# **Readme**
Install rust before start : https://www.rust-lang.org/fr/tools/install.

## Overview
brb-message-adversary repository is an implementation of a reliable broadcast algorithm defined here https://arxiv.org/abs/2205.09992. This client allows you to receive messages and broadcast them after several operations (cf: link). You can see a full diagram of it in *diagram.png*.


## Launch client
The best way to use the client is to run the bash script *launch_processus.sh*. This script is designed for _**local**_ experimentation and will launch a process in a terminal for every socket address that is in *ressources/processus.txt*. Use one of these terminal to test the client. In order to use it well, please read the **Correct and Byzantine processes** section. Moreover, if you want to test the client in a network (not locally), you must have the same *config_file.txt* and *processus.txt*  files for every process in order to use it correctly.

But if you still want to launch a process manually, then :
- go to the project directory
- run "cargo run --release *id,ipv4:port*"
- *example : cargo run 0,127.0.0.1:8085*

**Warning : launch the client with socket addresses** *(id,ipv4:port)* **that are in** *ressources/processus.txt* **only.**

## Command
When you launch the client into a terminal, it waits for a message to come or for a command by the user. You can broadcast a message to all the network members by entering :
- broadcast *message*
- *example : broadcast helloworld!*
  
*Note: message length must be strictly under 32 characters*

## Correct and Byzantine processes
You can modify **t** *(byzantine)* in *ressources/config_file.txt* as well as the presence of a message adversary (be aware to keep an empty line at the end of the file).
Reminder : **n > 3t+2d** (cf: link)
<br/><br/>

    NB : the broadcast function is designed to send the message to all the network members that are in the file "ressources/processus.txt". Feel free to modify this file if you want to, but the maximum number of processus that you can have is 255.