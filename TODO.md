# TODO
## User Management
- [ ] Allow the server to have a password so not anyone can join
- [ ] Allow users to set a color to their cursor

## File Management
- [ ] Implement some sort of .gitignore for files that don't want to be shared in the network
- [X] Create some sort of LocalPath struct to avoid sending PathBuf's over the network (possible LFI)
- [ ] Use u8 for tracking file content instead of string (faster and works better with binaries and shit like that)
- [ ] Search optimizations for FileTracking, maybe use async or something like that?

## File editing
- [X] Send the content of the files to the client when a user joins to actually edit them
    - [ ] Send them compressed and let the client decompress them for more network performance
- [X] Allow the users to send changes to files, containing only the parts changed and not the entire new file produced.
- [ ] Store the changes made by each user in a undo tree
    - [ ] Allow to remove specific (or all) changes made by a user
    - [ ] Allow to return to an earlier state of the file
    - [ ] Option for removing all changes made

## Options for the server CLI
- [ ] Initial args
    - [ ] path to shared directory/file (if none provided, just use the current working directory)
    - [ ] (optional) password
    - [ ] (optional) .fileignore => Just a file in the directory that's gonna be 
            shared with the files/directories that don't want to be shared accross the clients
- [ ] Commands while server is executing
    - [ ] ban => Ban user (By socket id or by username, haven't decided yet)
    - [ ] passwd => Change password
    - **UNDO STUFF GOES HERE**

## Docs
- [X] Create a small documentation page containing the messages the server accepts and returns

## Tests
- [ ] Set up proper testing on the project
    - [X] File Tracking
    - [ ] The server itself???

