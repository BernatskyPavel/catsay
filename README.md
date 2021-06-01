catsay - tool that makes a cat speak your message.

USAGE:
    catsay-bpv [FLAGS] [OPTIONS] [message]

FLAGS:
    -d, --dead       Make the cat appear dead
    -h, --help       Prints help information
    -i, --stdin      Read the message from STDIN instead of the argument
    -V, --version    Prints version information

OPTIONS:
    -f, --file <catfile>    Load the cat picture from specified file
    -w, --width <width>     Max length of one row of symbols [default: 40]  // Length will be limited to console length. 

ARGS:
    <message>    What does the cat say? [default: Meow!]

Cat will be placed exactly in the middle under the message.